import { ApiResponse, callApi } from ".";
import {
  AuthAccountInfo,
  CheckAuthRequest,
  CheckAuthResponse,
  InitAuthResponse,
  LocalAuthRequest,
  Provider,
} from "../api_types";

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
  const requestData: CheckAuthRequest = {
    callbackToken,
  };

  const response = await callApi<CheckAuthResponse>(
    "auth/check",
    "POST",
    requestData
  );

  return response;
}

export async function loginLocal(
  username: string,
  password: string
): Promise<ApiResponse<AuthAccountInfo>> {
  const requestData: LocalAuthRequest = {
    username,
    password,
  };

  const response = callApi<AuthAccountInfo>("auth/local", "POST", requestData);
  return response;
}
