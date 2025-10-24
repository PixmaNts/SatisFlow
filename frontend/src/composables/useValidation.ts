import { ref, reactive, computed } from 'vue';
import type {
  ValidationRule,
  ValidationResult,
  ValidationState,
  FieldValidationConfig,
  FormValidationOptions,
  ValidationContext,
  OverclockValidationOptions,
  SomersloopValidationOptions,
  MachineCountValidationOptions,
  FactoryNameValidationOptions,
  LogisticsEndpointsValidationOptions
} from '@/types/validation';
import { defaultValidationMessages } from '@/types/validation';

export function useValidation(
  fieldConfigs: FieldValidationConfig = {},
  options: FormValidationOptions = {},
  context: ValidationContext = {}
) {
  // Default options
  const defaultOptions: Required<FormValidationOptions> = {
    validateOnBlur: true,
    validateOnChange: false,
    validateOnSubmit: true,
    stopOnFirstError: false
  };

  const mergedOptions = { ...defaultOptions, ...options };

  // Reactive validation state
  const validationState = reactive<ValidationState>({
    errors: {},
    isValid: true,
    isDirty: false,
    isValidating: false
  });

  // Track which fields have been touched
  const touchedFields = ref<Set<string>>(new Set());

  // Track validation promises for async validation
  const validationPromises = ref<Map<string, Promise<ValidationResult>>>(new Map());

  // Computed property to check if there are any errors
  const hasErrors = computed(() => {
    return Object.keys(validationState.errors).some(
      field => (validationState.errors[field] || []).length > 0
    );
  });

  // Computed property to get errors for a specific field
  const getFieldErrors = (fieldName: string) => {
    return validationState.errors[fieldName] || [];
  };

  // Computed property to check if a specific field has errors
  const fieldHasErrors = (fieldName: string) => {
    return getFieldErrors(fieldName).length > 0;
  };

  // Computed property to check if a specific field is valid
  const isFieldValid = (fieldName: string) => {
    return touchedFields.value.has(fieldName) && !fieldHasErrors(fieldName);
  };

  // Computed property to check if a specific field is invalid
  const isFieldInvalid = (fieldName: string) => {
    return touchedFields.value.has(fieldName) && fieldHasErrors(fieldName);
  };

  // Validate a single field
  const validateField = async (fieldName: string, value: unknown): Promise<ValidationResult> => {
    const rules = fieldConfigs[fieldName];
    if (!rules || rules.length === 0) {
      return { isValid: true };
    }

    // Cancel any existing validation for this field
    if (validationPromises.value.has(fieldName)) {
      // Note: We can't actually cancel promises, but we can ignore their results
      validationPromises.value.delete(fieldName);
    }

    // Clear previous errors for this field
    delete validationState.errors[fieldName];

    const errors: string[] = [];
    let isValid = true;

    // Run validation rules
    for (const rule of rules) {
      try {
        const result = await Promise.resolve(rule.validator(value));

        if (!result.isValid) {
          isValid = false;
          const errorMessage = result.message || rule.message || defaultValidationMessages.custom;
          errors.push(errorMessage);

          if (mergedOptions.stopOnFirstError) {
            break;
          }
        }
      } catch (error) {
        isValid = false;
        const errorMessage = error instanceof Error ? error.message : defaultValidationMessages.custom;
        errors.push(errorMessage);

        if (mergedOptions.stopOnFirstError) {
          break;
        }
      }
    }

    // Update validation state
    if (errors.length > 0) {
      validationState.errors[fieldName] = errors;
    }

    // Update overall validity
    validationState.isValid = !hasErrors.value;

    return { isValid, message: errors[0] };
  };

  // Validate the entire form
  const validateForm = async (formData?: Record<string, unknown>): Promise<boolean> => {
    validationState.isValidating = true;
    validationState.isDirty = true;

    const data = formData || context.formData || {};
    const validationResults: Promise<ValidationResult>[] = [];

    // Validate all fields
    for (const fieldName of Object.keys(fieldConfigs)) {
      const value = data[fieldName];
      touchedFields.value.add(fieldName);

      const validationPromise = validateField(fieldName, value);
      validationPromises.value.set(fieldName, validationPromise);
      validationResults.push(validationPromise);
    }

    // Wait for all validations to complete
    await Promise.allSettled(validationResults);

    // Clear validation promises
    validationPromises.value.clear();

    validationState.isValidating = false;
    return validationState.isValid;
  };

  // Reset validation state
  const resetValidation = () => {
    validationState.errors = {};
    validationState.isValid = true;
    validationState.isDirty = false;
    validationState.isValidating = false;
    touchedFields.value.clear();
    validationPromises.value.clear();
  };

  // Reset validation for a specific field
  const resetFieldValidation = (fieldName: string) => {
    delete validationState.errors[fieldName];
    touchedFields.value.delete(fieldName);
    validationPromises.value.delete(fieldName);
    validationState.isValid = !hasErrors.value;
  };

  // Mark a field as touched (triggering validation display)
  const touchField = (fieldName: string) => {
    touchedFields.value.add(fieldName);
    validationState.isDirty = true;
  };

  // Mark all fields as touched
  const touchAllFields = () => {
    Object.keys(fieldConfigs).forEach(fieldName => {
      touchedFields.value.add(fieldName);
    });
    validationState.isDirty = true;
  };

  // Set field as dirty and validate if configured
  const setFieldValue = async (fieldName: string, value: unknown) => {
    touchedFields.value.add(fieldName);
    validationState.isDirty = true;

    if (mergedOptions.validateOnChange) {
      await validateField(fieldName, value);
    }
  };

  // Handle field blur event
  const handleFieldBlur = async (fieldName: string, value: unknown) => {
    touchedFields.value.add(fieldName);

    if (mergedOptions.validateOnBlur) {
      await validateField(fieldName, value);
    }
  };

  // Handle form submission
  const handleSubmit = async (formData: Record<string, unknown>, submitCallback: () => void | Promise<void>) => {
    if (mergedOptions.validateOnSubmit) {
      const isValid = await validateForm(formData);
      if (!isValid) {
        touchAllFields();
        return false;
      }
    }

    await submitCallback();
    return true;
  };

  return {
    // State
    validationState,
    touchedFields,

    // Computed
    hasErrors,
    isValid: computed(() => validationState.isValid),
    isDirty: computed(() => validationState.isDirty),
    isValidating: computed(() => validationState.isValidating),

    // Field-specific computed
    getFieldErrors,
    fieldHasErrors,
    isFieldValid,
    isFieldInvalid,

    // Methods
    validateField,
    validateForm,
    resetValidation,
    resetFieldValidation,
    touchField,
    touchAllFields,
    setFieldValue,
    handleFieldBlur,
    handleSubmit
  };
}

// Specific validation functions for Satisflow
export const validateOverclock = (options: OverclockValidationOptions = {}) => {
  const { min = 0.000, max = 250.000, precision = 3 } = options;

  return (value: unknown): ValidationResult => {
    if (value === null || value === undefined || value === '') {
      return { isValid: true }; // Optional field
    }

    const numValue = Number(value);

    if (isNaN(numValue)) {
      return {
        isValid: false,
        message: defaultValidationMessages.number
      };
    }

    if (numValue < min || numValue > max) {
      return {
        isValid: false,
        message: defaultValidationMessages.overclockRange
      };
    }

    // Check precision
    const decimalPlaces = numValue.toString().split('.')[1]?.length || 0;
    if (decimalPlaces > precision) {
      return {
        isValid: false,
        message: defaultValidationMessages.overclockPrecision
      };
    }

    return { isValid: true };
  };
};

export const validateSomersloop = (options: SomersloopValidationOptions) => {
  const { machineType, maxSomersloops } = options;

  return (value: unknown): ValidationResult => {
    if (value === null || value === undefined || value === '') {
      return { isValid: true }; // Optional field
    }

    const numValue = Number(value);

    if (isNaN(numValue) || numValue < 0 || !Number.isInteger(numValue)) {
      return {
        isValid: false,
        message: defaultValidationMessages.integer
      };
    }

    const maxAllowed = maxSomersloops[machineType] || 0;
    if (numValue > maxAllowed) {
      return {
        isValid: false,
        message: defaultValidationMessages.somersloopLimit
          .replace('{max}', maxAllowed.toString())
          .replace('{machineType}', machineType)
      };
    }

    return { isValid: true };
  };
};

export const validateMachineCount = (options: MachineCountValidationOptions = {}) => {
  const { min = 1, max } = options;

  return (value: unknown): ValidationResult => {
    if (value === null || value === undefined || value === '') {
      return { isValid: false, message: defaultValidationMessages.required };
    }

    const numValue = Number(value);

    if (isNaN(numValue) || numValue < 0 || !Number.isInteger(numValue)) {
      return {
        isValid: false,
        message: defaultValidationMessages.integer
      };
    }

    if (numValue < min) {
      return {
        isValid: false,
        message: defaultValidationMessages.machineCountPositive
      };
    }

    if (max !== undefined && numValue > max) {
      return {
        isValid: false,
        message: defaultValidationMessages.max.replace('{max}', max.toString())
      };
    }

    return { isValid: true };
  };
};

export const validateFactoryName = (options: FactoryNameValidationOptions) => {
  const { existingNames, currentName = '' } = options;

  return (value: unknown): ValidationResult => {
    if (value === null || value === undefined || value === '') {
      return { isValid: false, message: defaultValidationMessages.required };
    }

    const strValue = String(value).trim();

    if (strValue.length === 0) {
      return { isValid: false, message: defaultValidationMessages.required };
    }

    // Check for uniqueness (excluding current name if editing)
    const isDuplicate = existingNames.some(name =>
      name.toLowerCase() === strValue.toLowerCase() && name !== currentName
    );

    if (isDuplicate) {
      return {
        isValid: false,
        message: defaultValidationMessages.factoryNameUnique
      };
    }

    return { isValid: true };
  };
};

export const validateLogisticsEndpoints = (options: LogisticsEndpointsValidationOptions) => {
  const { sourceFactoryId, destinationFactoryId } = options;

  return (): ValidationResult => {
    if (sourceFactoryId === destinationFactoryId) {
      return {
        isValid: false,
        message: defaultValidationMessages.logisticsSameEndpoint
      };
    }

    return { isValid: true };
  };
};

// Basic validation rules
export const required = (message?: string): ValidationRule => ({
  name: 'required',
  validator: (value: unknown): ValidationResult => {
    if (value === null || value === undefined || value === '') {
      return { isValid: false, message: message || defaultValidationMessages.required };
    }
    return { isValid: true };
  },
  message
});

export const min = (minValue: number, message?: string): ValidationRule => ({
  name: 'min',
  validator: (value: unknown): ValidationResult => {
    const numValue = Number(value);
    if (isNaN(numValue) || numValue < minValue) {
      return {
        isValid: false,
        message: message || defaultValidationMessages.min.replace('{min}', minValue.toString())
      };
    }
    return { isValid: true };
  },
  message
});

export const max = (maxValue: number, message?: string): ValidationRule => ({
  name: 'max',
  validator: (value: unknown): ValidationResult => {
    const numValue = Number(value);
    if (isNaN(numValue) || numValue > maxValue) {
      return {
        isValid: false,
        message: message || defaultValidationMessages.max.replace('{max}', maxValue.toString())
      };
    }
    return { isValid: true };
  },
  message
});

export const minLength = (minLength: number, message?: string): ValidationRule => ({
  name: 'minLength',
  validator: (value: unknown): ValidationResult => {
    const strValue = String(value || '');
    if (strValue.length < minLength) {
      return {
        isValid: false,
        message: message || defaultValidationMessages.minLength.replace('{min}', minLength.toString())
      };
    }
    return { isValid: true };
  },
  message
});

export const maxLength = (maxLength: number, message?: string): ValidationRule => ({
  name: 'maxLength',
  validator: (value: unknown): ValidationResult => {
    const strValue = String(value || '');
    if (strValue.length > maxLength) {
      return {
        isValid: false,
        message: message || defaultValidationMessages.maxLength.replace('{max}', maxLength.toString())
      };
    }
    return { isValid: true };
  },
  message
});

export const pattern = (regex: RegExp, message?: string): ValidationRule => ({
  name: 'pattern',
  validator: (value: unknown): ValidationResult => {
    const strValue = String(value || '');
    if (!regex.test(strValue)) {
      return {
        isValid: false,
        message: message || defaultValidationMessages.pattern
      };
    }
    return { isValid: true };
  },
  message
});

export const number = (message?: string): ValidationRule => ({
  name: 'number',
  validator: (value: unknown): ValidationResult => {
    if (value === null || value === undefined || value === '') {
      return { isValid: true }; // Optional field
    }
    const numValue = Number(value);
    if (isNaN(numValue)) {
      return {
        isValid: false,
        message: message || defaultValidationMessages.number
      };
    }
    return { isValid: true };
  },
  message
});

export const integer = (message?: string): ValidationRule => ({
  name: 'integer',
  validator: (value: unknown): ValidationResult => {
    if (value === null || value === undefined || value === '') {
      return { isValid: true }; // Optional field
    }
    const numValue = Number(value);
    if (isNaN(numValue) || !Number.isInteger(numValue)) {
      return {
        isValid: false,
        message: message || defaultValidationMessages.integer
      };
    }
    return { isValid: true };
  },
  message
});

export const positive = (message?: string): ValidationRule => ({
  name: 'positive',
  validator: (value: unknown): ValidationResult => {
    if (value === null || value === undefined || value === '') {
      return { isValid: true }; // Optional field
    }
    const numValue = Number(value);
    if (isNaN(numValue) || numValue <= 0) {
      return {
        isValid: false,
        message: message || defaultValidationMessages.positive
      };
    }
    return { isValid: true };
  },
  message
});

export const custom = (validator: (value: unknown) => ValidationResult | Promise<ValidationResult>, message?: string): ValidationRule => ({
  name: 'custom',
  validator,
  message
});
