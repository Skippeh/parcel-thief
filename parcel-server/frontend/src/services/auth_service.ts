import { ApiResponse, callApi } from ".";

export enum Provider {
  Steam = "steam",
  Epic = "epic",
}

export interface InitAuthResponse {
  token: string;
  redirectUrl: string;
}

export interface CheckAuthResponse {
  success?: {
    authToken: string;
    name: string;
    avatarUrl: string;
  };
  failure?: {
    error: string;
  };
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

export async function checkAuthResult(
  callbackToken: string
): Promise<ApiResponse<CheckAuthResponse>> {
  const requestData = {
    callbackToken,
  };

  const response = await callApi<CheckAuthResponse>(
    "auth/check",
    "POST",
    requestData
  );

  return response;
}
