// Validation types for Satisflow frontend

export interface ValidationRule {
  name: string;
  validator: (value: unknown) => ValidationResult | Promise<ValidationResult>;
  message?: string;
}

export interface ValidationResult {
  isValid: boolean;
  message?: string;
}

export interface ValidationErrors {
  [fieldName: string]: string[];
}

export interface ValidationState {
  errors: ValidationErrors;
  isValid: boolean;
  isDirty: boolean;
  isValidating: boolean;
}

export interface FieldValidationConfig {
  [fieldName: string]: ValidationRule[];
}

export interface FormValidationOptions {
  validateOnBlur?: boolean;
  validateOnChange?: boolean;
  validateOnSubmit?: boolean;
  stopOnFirstError?: boolean;
}

// Specific validation types for Satisflow
export interface OverclockValidationOptions {
  min?: number;
  max?: number;
  precision?: number;
}

export interface SomersloopValidationOptions {
  machineType: string;
  maxSomersloops: Record<string, number>;
}

export interface MachineCountValidationOptions {
  min?: number;
  max?: number;
}

export interface FactoryNameValidationOptions {
  existingNames: string[];
  currentName?: string;
}

export interface LogisticsEndpointsValidationOptions {
  sourceFactoryId: string | number;
  destinationFactoryId: string | number;
}

// Validation message types for internationalization
export interface ValidationMessages {
  required: string;
  min: string;
  max: string;
  minLength: string;
  maxLength: string;
  pattern: string;
  number: string;
  integer: string;
  positive: string;
  overclockRange: string;
  overclockPrecision: string;
  somersloopLimit: string;
  machineCountPositive: string;
  factoryNameUnique: string;
  logisticsSameEndpoint: string;
  custom: string;
}

// Default validation messages (can be replaced with i18n)
export const defaultValidationMessages: ValidationMessages = {
  required: 'This field is required',
  min: 'Value must be at least {min}',
  max: 'Value must be at most {max}',
  minLength: 'Must be at least {min} characters',
  maxLength: 'Must be at most {max} characters',
  pattern: 'Invalid format',
  number: 'Must be a valid number',
  integer: 'Must be a whole number',
  positive: 'Must be a positive number',
  overclockRange: 'Overclock must be between 0.000 and 250.000',
  overclockPrecision: 'Overclock must have at most 3 decimal places',
  somersloopLimit: 'Maximum {max} somersloops allowed for {machineType}',
  machineCountPositive: 'Machine count must be a positive number',
  factoryNameUnique: 'Factory name must be unique',
  logisticsSameEndpoint: 'Source and destination factories must be different',
  custom: 'Invalid value'
};

// Validation context for complex validations
export interface ValidationContext {
  formData?: Record<string, unknown>;
  apiClient?: unknown;
  machineTypes?: Record<string, unknown>;
  factories?: Record<string, unknown>;
}
