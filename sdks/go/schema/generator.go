package schema

import (
	"fmt"
	"reflect"
	"strings"

	pb "github.com/riken127/graveyar_db/sdks/go/proto"
)

// Generate creates a protobuf Schema definition from a Go struct.
func Generate(v interface{}) (*pb.Schema, error) {
	t := reflect.TypeOf(v)
	// Dereference pointer if needed
	if t.Kind() == reflect.Ptr {
		t = t.Elem()
	}

	if t.Kind() != reflect.Struct {
		return nil, fmt.Errorf("schema generation requires a struct, got %s", t.Kind())
	}

	return generateStructSchema(t)
}

func generateStructSchema(t reflect.Type) (*pb.Schema, error) {
	schemaName := t.Name()
	fields := make(map[string]*pb.Field)

	for i := 0; i < t.NumField(); i++ {
		f := t.Field(i)
		
		// Skip unexported fields
		if f.PkgPath != "" {
			continue
		}

		fieldName := f.Name
		
		// Parse tags
		tag := f.Tag.Get("graveyard") // Placeholder for future complex tags
		_ = tag 
		jsonTag := f.Tag.Get("json")

		if jsonTag != "" && jsonTag != "-" {
			parts := strings.Split(jsonTag, ",")
			if parts[0] != "" {
				fieldName = parts[0]
			}
		}

		pbField, err := mapField(f.Type)
		if err != nil {
			return nil, fmt.Errorf("field %s: %w", f.Name, err)
		}
		
		fields[fieldName] = pbField
	}

	return &pb.Schema{
		Name:   schemaName,
		Fields: fields,
	}, nil
}

// mapField handles the creation of a Field, including processing nullability/pointers
func mapField(t reflect.Type) (*pb.Field, error) {
	nullable := false
	currentType := t
	
	if currentType.Kind() == reflect.Ptr {
		nullable = true
		currentType = currentType.Elem()
	}

	fieldType, err := mapFieldType(currentType)
	if err != nil {
		return nil, err
	}

	return &pb.Field{
		FieldType:       fieldType,
		Nullable:        nullable,
		OverridesOnNull: false, 
	}, nil
}

// mapFieldType handles the creation of the FieldType definitions (recursive)
func mapFieldType(t reflect.Type) (*pb.FieldType, error) {
	ft := &pb.FieldType{}

	switch t.Kind() {
	case reflect.String:
		ft.Kind = &pb.FieldType_Primitive_{Primitive: pb.FieldType_STRING}
	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64, 
		 reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64, 
		 reflect.Float32, reflect.Float64:
		ft.Kind = &pb.FieldType_Primitive_{Primitive: pb.FieldType_NUMBER}
	case reflect.Bool:
		ft.Kind = &pb.FieldType_Primitive_{Primitive: pb.FieldType_BOOLEAN}
	case reflect.Slice, reflect.Array:
		// Handle []byte as Primitive String/Blob? Or Array of Numbers?
		// Usually []byte is treated as bytes. 
		if t.Elem().Kind() == reflect.Uint8 {
			// Special case for byte slice -> typically binary data. 
			// But our proto Primitive only has NUMBER, STRING, BOOLEAN.
			// Let's map to STRING (base64) or fallback to Array of Numbers.
			// For now, let's treat as Array of Numbers (uint8) for strictness, 
			// or if we add BYTES primitive later. 
			// Let's just treat as Array of logic.
		}
		
		elemType, err := mapFieldType(t.Elem())
		if err != nil {
			return nil, err
		}
		ft.Kind = &pb.FieldType_ArrayDef{ArrayDef: &pb.FieldType_Array{ElementType: elemType}}
		
	case reflect.Struct:
		subSchema, err := generateStructSchema(t)
		if err != nil {
			return nil, err
		}
		ft.Kind = &pb.FieldType_SubSchema{SubSchema: subSchema}
		
	default:
		return nil, fmt.Errorf("unsupported type: %s", t.Kind())
	}

	return ft, nil
}
