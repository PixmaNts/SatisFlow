import { describe, it, expect, beforeEach } from 'vitest';
import { useValidation, validateOverclock, validateSomersloop, validateMachineCount, validateFactoryName } from '../useValidation';
import type { FieldValidationConfig } from '@/types/validation';

describe('useValidation', () => {
  let fieldConfigs: FieldValidationConfig;

  beforeEach(() => {
    fieldConfigs = {
      name: [
        {
          name: 'required',
          validator: (value) => {
            if (!value || (typeof value === 'string' && value.trim() === '')) {
              return { isValid: false, message: 'Name is required' };
            }
            return { isValid: true };
          }
        }
      ],
      overclock: [
        {
          name: 'overclock',
          validator: validateOverclock()
        }
      ]
    };
  });

  it('should initialize with valid state', () => {
    const { isValid, hasErrors, validationState } = useValidation(fieldConfigs);

    expect(isValid.value).toBe(true);
    expect(hasErrors.value).toBe(false);
    expect(validationState.errors).toEqual({});
  });

  it('should validate a single field', async () => {
    const { validateField, getFieldErrors } = useValidation(fieldConfigs);

    // Test valid value
    let result = await validateField('name', 'Test Factory');
    expect(result.isValid).toBe(true);
    expect(getFieldErrors('name')).toEqual([]);

    // Test invalid value
    result = await validateField('name', '');
    expect(result.isValid).toBe(false);
    expect(getFieldErrors('name')).toContain('Name is required');
  });

  it('should validate overclock values', async () => {
    const validator = validateOverclock();

    // Valid values
    expect(validator(100).isValid).toBe(true);
    expect(validator(0).isValid).toBe(true);
    expect(validator(250).isValid).toBe(true);
    expect(validator(123.456).isValid).toBe(true);

    // Invalid values
    expect(validator(-1).isValid).toBe(false);
    expect(validator(251).isValid).toBe(false);
    expect(validator(123.4567).isValid).toBe(false); // Too many decimals
    expect(validator('invalid').isValid).toBe(false);
  });

  it('should validate somersloop counts', () => {
    const machineTypes = {
      Constructor: 1,
      Assembler: 2,
      Manufacturer: 4
    };

    const constructorValidator = validateSomersloop({
      machineType: 'Constructor',
      maxSomersloops: machineTypes
    });

    const assemblerValidator = validateSomersloop({
      machineType: 'Assembler',
      maxSomersloops: machineTypes
    });

    // Constructor validations
    expect(constructorValidator(0).isValid).toBe(true);
    expect(constructorValidator(1).isValid).toBe(true);
    expect(constructorValidator(2).isValid).toBe(false);

    // Assembler validations
    expect(assemblerValidator(0).isValid).toBe(true);
    expect(assemblerValidator(2).isValid).toBe(true);
    expect(assemblerValidator(3).isValid).toBe(false);
  });

  it('should validate machine counts', () => {
    const validator = validateMachineCount();

    // Valid values
    expect(validator(1).isValid).toBe(true);
    expect(validator(10).isValid).toBe(true);

    // Invalid values
    expect(validator(0).isValid).toBe(false);
    expect(validator(-1).isValid).toBe(false);
    expect(validator('invalid').isValid).toBe(false);
  });

  it('should validate factory names', () => {
    const existingNames = ['Factory A', 'Factory B'];

    // New factory validation
    const newFactoryValidator = validateFactoryName({
      existingNames
    });

    expect(newFactoryValidator('Factory C').isValid).toBe(true);
    expect(newFactoryValidator('Factory A').isValid).toBe(false);
    expect(newFactoryValidator('').isValid).toBe(false);

    // Editing existing factory
    const editFactoryValidator = validateFactoryName({
      existingNames,
      currentName: 'Factory A'
    });

    expect(editFactoryValidator('Factory A').isValid).toBe(true);
    expect(editFactoryValidator('Factory B').isValid).toBe(false);
  });

  it('should validate entire form', async () => {
    const { validateForm, isValid, getFieldErrors } = useValidation(fieldConfigs);

    const formData = {
      name: 'Test Factory',
      overclock: 150
    };

    const isValidForm = await validateForm(formData);

    expect(isValidForm).toBe(true);
    expect(isValid.value).toBe(true);
    expect(getFieldErrors('name')).toEqual([]);
    expect(getFieldErrors('overclock')).toEqual([]);
  });

  it('should handle form validation errors', async () => {
    const { validateForm, isValid, getFieldErrors } = useValidation(fieldConfigs);

    const formData = {
      name: '',
      overclock: 300 // Invalid overclock
    };

    const isValidForm = await validateForm(formData);

    expect(isValidForm).toBe(false);
    expect(isValid.value).toBe(false);
    expect(getFieldErrors('name')).toContain('Name is required');
    expect(getFieldErrors('overclock')).toContain('Overclock must be between 0.000 and 250.000');
  });

  it('should reset validation state', () => {
    const { resetValidation, validationState } = useValidation(fieldConfigs);

    // Simulate some errors
    validationState.errors = { name: ['Error'] };
    validationState.isValid = false;
    validationState.isDirty = true;

    resetValidation();

    expect(validationState.errors).toEqual({});
    expect(validationState.isValid).toBe(true);
    expect(validationState.isDirty).toBe(false);
  });
});
