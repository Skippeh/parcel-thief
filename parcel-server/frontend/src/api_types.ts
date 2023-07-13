// AUTO-GENERATED by typescript-type-def

export type Provider=("steam"|"epic");
export type AuthRequest={"provider":Provider;};
export type InitAuthResponse={"redirectUrl":string;};
export type CheckAuthRequest={"callbackToken":string;};
export type CheckAuthResponse=(({"type":"success";}&{"name":string;"avatarUrl":string;"authToken":string;})|({"type":"failure";}&{"error":string;}));
export type I64=number;
export type JwtPayload={"expiresAt":I64;"accountId":string;};
