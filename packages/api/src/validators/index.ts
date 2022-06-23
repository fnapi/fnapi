export interface Validator<T> {
  validate(fieldName: string, value: T): Error | undefined;
}

export function custom<T>(
  validate: (fieldName: string, val: T) => Error | undefined
): Validator<T> {
  return {
    validate,
  };
}

/**
 * Core validation logics, which can be executed from client side too.
 */
class CoreValidator<T> implements Validator<T> {
  constructor(private fn: (fieldName: string, val: T) => Error | undefined) {}

  validate(fieldName: string, value: T): Error | undefined {
    return this.fn(fieldName, value);
  }
}

export const required: CoreValidator<any> = new CoreValidator(
  (fieldName, val: any) => {
    if (val === "") {
      return new Error(`${fieldName} is required`);
    }
  }
);

export const email: CoreValidator<string> = new CoreValidator(
  (fieldName, val) => {
    if (val === "") {
      return new Error(`${fieldName} is required`);
    }

    if (!val.includes("@")) {
      return new Error(`${fieldName} should be a valid email`);
    }
  }
);

export function minLength(length: number): CoreValidator<string> {
  return new CoreValidator((fieldName, val) => {
    if (val.length < length) {
      return new Error(
        `${fieldName} is too short. It should be at least ${length} characters long.`
      );
    }
  });
}

export function maxLength(length: number): CoreValidator<string> {
  return new CoreValidator((fieldName, val) => {
    if (val.length > length) {
      return new Error(
        `${fieldName} is too long. It should be at most ${length} characters long.`
      );
    }
  });
}
