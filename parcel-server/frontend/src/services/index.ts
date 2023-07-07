export interface ApiResponse<T> {
  data: T | null;
  statusCode: number;
  error: string | null;
}

export function getApiUrl(apiRoute: string): string {
  return `/frontend/api/${apiRoute}`;
}

export async function callApi<T>(
  route: string,
  method: string,
  requestData: any | null
): Promise<ApiResponse<T>> {
  let apiResponse: ApiResponse<T>;

  try {
    const response = await fetch(getApiUrl(route), {
      method,
      body: requestData == null ? null : JSON.stringify(requestData),
      headers: {
        "Content-Type": "application/json",
      },
    });

    apiResponse = response.json() as unknown as ApiResponse<T>;
  } catch (err) {
    console.error("Api request failed:", err);
    apiResponse = {
      data: null,
      statusCode: -1,
      error: "Could not send api request",
    };
  }

  return apiResponse;
}
