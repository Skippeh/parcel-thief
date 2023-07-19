import { ApiResponse, callApi } from ".";
import {
  FrontendAccount,
  FrontendPermissions,
  ListAccountsResponse,
  ListAccountsType,
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
