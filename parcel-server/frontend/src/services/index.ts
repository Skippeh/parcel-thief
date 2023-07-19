export interface FieldError {
  code: string;
}

export type FormErrors = {
  [key: string]: FieldError[];
};

export type MappedFormErrors = {
  [key: string]: {
    [key: string]: boolean;
  };
};

export function mapFormErrors(
  errors: FormErrors | null | undefined
): MappedFormErrors {
  let result = {};

  if (errors == null) {
    return result;
  }

  Object.keys(errors).forEach((key) => {
    result[key] = {};
    errors[key].forEach((error) => {
      result[key][error.code] = true;
    });
  });

  return result;
}

export interface ApiResponse<T> {
  data: T | null;
  statusCode: number;
  error: string | null;
  formErrors: FormErrors | null;
}

let authToken: string | null = null;

export function setAuthToken(token: string | null) {
  authToken = token;
}

export function getApiUrl(apiRoute: string): string {
  return `/frontend/api/${apiRoute}`;
}

export async function callApi<T>(
  route: string,
  method: string,
  requestData?: any
): Promise<ApiResponse<T>> {
  let apiResponse: ApiResponse<T>;

  try {
    const headers = {
      "Content-Type": "application/json",
    };

    // If we have an auth token, set Authorization header
    if (authToken != null) {
      headers["Authorization"] = `Bearer ${authToken}`;
    }

    const response = await fetch(getApiUrl(route), {
      method,
      body: requestData == null ? null : JSON.stringify(requestData),
      headers,
    });

    apiResponse = (await response.json()) as ApiResponse<T>;
  } catch (err) {
    console.error("Api request failed:", err);
    apiResponse = {
      data: null,
      statusCode: -1,
      error: "Could not send api request or read response",
    };
  }

  return apiResponse;
}
