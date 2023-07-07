import { ApiResponse, callApi } from ".";

export enum Provider {
  Steam = "steam",
  Epic = "epic",
}

export interface InitAuthResponse {
  token: string;
  redirectUrl: string;
}

export async function login(
  provider: Provider
): Promise<ApiResponse<InitAuthResponse>> {
  const requestData = {
    provider,
  };

  const response = await callApi<InitAuthResponse>("auth", "POST", requestData);
  return response;
}
