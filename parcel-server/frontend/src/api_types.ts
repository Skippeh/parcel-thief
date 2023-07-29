// AUTO-GENERATED by typescript-type-def

export type Provider=("steam"|"epic");
export type AuthRequest={"provider":Provider;};
export type LocalAuthRequest={"username":string;"password":string;};
export type InitAuthResponse={"redirectUrl":string;};
export type CheckAuthRequest={"callbackToken":string;};
export type FrontendPermissions=("manageAccounts"|"manageServerSettings");
export type AuthAccountInfo={"name":string;"avatarUrl":(string|null);"authToken":string;"gameAccountId":(string|null);"permissions":(FrontendPermissions)[];};
export type CheckAuthResponse=(({"type":"success";}&AuthAccountInfo)|({"type":"failure";}&{"error":string;}));
export type I64=number;
export type JwtPayload={"expiresAt":I64;
/**
 * Frontend account id, not game account id
 */
"accountId":I64;};
export type I32=number;
export type SharedCargoListItem={"name":string;"amount":I32;"category":string;"location":string;"creator":string;};
export type ListSharedCargoResponse={"baggages":(SharedCargoListItem)[];};
export type LostCargoListItem={"name":string;"amount":I32;"category":string;"location":string;"endLocation":string;"creator":string;};
export type ListLostCargoResponse={"baggages":(LostCargoListItem)[];};
export type WastedCargoListItem={"name":string;"category":string;"broken":boolean;"location":string;"creator":string;};
export type ListWastedCargoResponse={"baggages":(WastedCargoListItem)[];};
export type ListAccountsType=("frontend"|"game");
export type FrontendAccountListItem={"id":I64;"gameId":(string|null);"name":string;"permissions":(FrontendPermissions)[];};
export type GameAccountListItem={"frontendId":(I64|null);"gameId":string;"name":string;"provider":Provider;"providerId":string;"lastLogin":string;};
export type ListAccountsResponse=(({"type":"frontend";}&{"accounts":(FrontendAccountListItem)[];})|({"type":"game";}&{"accounts":(GameAccountListItem)[];}));
export type ProviderConnection={"provider":Provider;"providerId":string;"name":(string|null);};
export type LocalAccount={"username":string;};
export type FrontendAccount={"id":I64;"gameId":(string|null);"permissions":(FrontendPermissions)[];"providerConnection":(ProviderConnection|null);"localAccount":(LocalAccount|null);};
export type SetAccountPermissionsRequest={"permissions":(FrontendPermissions)[];};
export type CreateCredentialsRequest={"username":string;"password":string;};
export type ResetPasswordRequest={"currentPassword":(string|null);"newPassword":string;"logoutSessions":boolean;};
export type CreateFrontendAccountRequest=(({"type":"withCredentials";}&CreateCredentialsRequest)|({"type":"withProvider";}&{"provider":Provider;"providerId":string;}));
