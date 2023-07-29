import { ApiResponse, callApi } from ".";
import {
  CreateCredentialsRequest,
  CreateFrontendAccountRequest,
  FrontendAccount,
  FrontendPermissions,
  ListAccountsResponse,
  ListAccountsType,
  LocalAccount,
  ResetPasswordRequest,
  SetAccountPermissionsRequest,
} from "../api_types";

export async function getAccounts<
  T extends ListAccountsType,
  V extends { type: T } & ListAccountsResponse
>(type: T): Promise<ApiResponse<V>> {
  const query = `?accountsType=${encodeURIComponent(type)}`;
  return await callApi(`accounts${query}`, "GET");
}

export async function getFrontendAccount(
  id: number
): Promise<ApiResponse<FrontendAccount>> {
  return await callApi(`accounts/frontend/${id}`, "GET");
}

export function permissionToReadableString(
  permission: FrontendPermissions | undefined | null
): string {
  switch (permission) {
    case "manageAccounts":
      return "Manage accounts";
    case "manageServerSettings":
      return "Manage server settings";
    default:
      return permission ?? "Unknown";
  }
}

export function setAccountPermissions(
  accountId: number,
  permissions: FrontendPermissions[]
): Promise<ApiResponse<FrontendPermissions[]>> {
  const requestData: SetAccountPermissionsRequest = {
    permissions,
  };

  return callApi(
    `accounts/frontend/${accountId}/permissions`,
    "PUT",
    requestData
  );
}

export function createLocalAccountFor(
  accountId: number,
  username: string,
  password: string
): Promise<ApiResponse<LocalAccount>> {
  const requestData: CreateCredentialsRequest = {
    username,
    password,
  };

  return callApi(
    `accounts/createCredentials/${accountId}`,
    "POST",
    requestData
  );
}

export function resetAccountPassword(
  accountId: number,
  currentPassword: string | null,
  newPassword: string,
  logoutSessions: boolean
): Promise<ApiResponse<void>> {
  const requestData: ResetPasswordRequest = {
    currentPassword,
    newPassword,
    logoutSessions,
  };

  return callApi(`accounts/resetPassword/${accountId}`, "POST", requestData);
}

export function createFrontendAccount(
  request: CreateFrontendAccountRequest
): Promise<ApiResponse<number>> {
  return callApi("accounts/createFrontendAccount", "POST", request);
}
