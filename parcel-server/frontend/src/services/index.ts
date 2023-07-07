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
  let response = await fetch(getApiUrl(route), {
    method,
    body: requestData == null ? null : JSON.stringify(requestData),
    headers: {
      "Content-Type": "application/json",
    },
  });

  let apiResponse = response.json() as unknown as ApiResponse<T>;
  return apiResponse;
}
