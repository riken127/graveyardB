package com.eventstore.client.annotations;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

/**
 * Configures a field within a GraveyardEntity.
 */
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.FIELD)
public @interface GraveyardField {
    /**
     * Whether the field is nullable.
     */
    boolean nullable() default true; // Defaulting to true as Java objects are nullable

    /**
     * Whether null values should override existing values in partial updates.
     */
    boolean overridesOnNull() default false;

    // --- Constraints ---

    /**
     * Minimum numeric value (for numbers).
     */
    double min() default Double.NaN;

    /**
     * Maximum numeric value (for numbers).
     */
    double max() default Double.NaN;

    /**
     * Minimum string length.
     */
    int minLength() default -1;

    /**
     * Maximum string length.
     */
    int maxLength() default -1;

    /**
     * Regex pattern for string validation.
     */
    String regex() default "";
}
