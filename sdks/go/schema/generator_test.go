package schema

import (
	"testing"
	"encoding/json"
	
	pb "github.com/riken127/graveyar_db/sdks/go/proto"
)

type ValidationStruct struct {
	Name    string `json:"full_name"`
	Age     int
	Active  bool
	Tags    []string
	Address *AddressStruct
}

type AddressStruct struct {
	Street string
	City   string
}

func TestGenerate(t *testing.T) {
	v := ValidationStruct{}
	schema, err := Generate(v)
	if err != nil {
		t.Fatalf("Generate failed: %v", err)
	}

	// Basic check
	if schema.Name != "ValidationStruct" {
		t.Errorf("Expected schema name ValidationStruct, got %s", schema.Name)
	}

	// Check fields
	if len(schema.Fields) != 5 {
		t.Errorf("Expected 5 fields, got %d", len(schema.Fields))
	}

	// name field (mapped from json tag)
	if f, ok := schema.Fields["full_name"]; !ok {
		t.Errorf("Missing full_name field")
	} else {
		// Check type
		if _, ok := f.FieldType.Kind.(*pb.FieldType_Primitive_); !ok {
			t.Errorf("full_name should be primitive")
		}
	}

	// Address field (SubSchema + Nullable)
	if f, ok := schema.Fields["Address"]; !ok {
		t.Errorf("Missing Address field")
	} else {
		if !f.Nullable {
			t.Errorf("Address should be nullable pointer")
		}
		if _, ok := f.FieldType.Kind.(*pb.FieldType_SubSchema); !ok {
			t.Errorf("Address should be SubSchema")
		}
	}

	// Tags field (Array)
	if f, ok := schema.Fields["Tags"]; !ok {
		t.Errorf("Missing Tags field")
	} else {
		if arr, ok := f.FieldType.Kind.(*pb.FieldType_ArrayDef); !ok {
			t.Errorf("Tags should be Array")
		} else {
			// Check element type
			if _, ok := arr.ArrayDef.ElementType.Kind.(*pb.FieldType_Primitive_); !ok {
				t.Errorf("Tags element type should be primitive")
			}
		}
	}
	
	// Debug print
	b, _ := json.MarshalIndent(schema, "", "  ")
	t.Logf("Generated Schema: %s", string(b))
}
