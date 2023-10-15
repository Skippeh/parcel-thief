// AUTO-GENERATED by typescript-type-def

export type Provider = ("steam" | "epic");
export type AuthRequest = {
    "provider": Provider;
};
export type LocalAuthRequest = {
    "username": string;
    "password": string;
};
export type InitAuthResponse = {
    "redirectUrl": string;
};
export type CheckAuthRequest = {
    "callbackToken": string;
};
export type FrontendPermissions = ("manageAccounts" | "manageServerSettings");
export type AuthAccountInfo = {
    "name": string;
    "avatarUrl": (string | null);
    "authToken": string;
    "gameAccountId": (string | null);
    "permissions": (FrontendPermissions)[];
};
export type CheckAuthResponse = (({
    "type": "success";
} & AuthAccountInfo) | ({
    "type": "failure";
} & {
    "error": string;
}));
export type I64 = number;
export type JwtPayload = {
    "expiresAt": I64;

    /**
     * Frontend account id, not game account id
     */
    "accountId": I64;
};
export type I32 = number;
export type SharedCargoListItem = {
    "name": string;
    "amount": I32;
    "category": string;
    "location": string;
    "creator": string;
};
export type ListSharedCargoResponse = {
    "baggages": (SharedCargoListItem)[];
};
export type LostCargoListItem = {
    "name": string;
    "amount": I32;
    "category": string;
    "location": string;
    "endLocation": string;
    "creator": string;
};
export type ListLostCargoResponse = {
    "baggages": (LostCargoListItem)[];
};
export type WastedCargoListItem = {
    "name": string;
    "category": string;
    "broken": boolean;
    "location": string;
    "creator": string;
};
export type ListWastedCargoResponse = {
    "baggages": (WastedCargoListItem)[];
};
export type ListAccountsType = ("frontend" | "game");
export type FrontendAccountListItem = {
    "id": I64;
    "gameId": (string | null);
    "name": string;
    "permissions": (FrontendPermissions)[];
};
export type GameAccountListItem = {
    "frontendId": (I64 | null);
    "gameId": string;
    "name": string;
    "provider": Provider;
    "providerId": string;
    "lastLogin": string;
};
export type ListAccountsResponse = (({
    "type": "frontend";
} & {
    "accounts": (FrontendAccountListItem)[];
}) | ({
    "type": "game";
} & {
    "accounts": (GameAccountListItem)[];
}));
export type ProviderConnection = {
    "provider": Provider;
    "providerId": string;
    "name": (string | null);
};
export type LocalAccount = {
    "username": string;
};
export type FrontendAccount = {
    "id": I64;
    "gameId": (string | null);
    "permissions": (FrontendPermissions)[];
    "providerConnection": (ProviderConnection | null);
    "localAccount": (LocalAccount | null);
};
export type SetAccountPermissionsRequest = {
    "permissions": (FrontendPermissions)[];
};
export type CreateCredentialsRequest = {
    "username": string;
    "password": string;
};
export type ResetPasswordRequest = {
    "currentPassword": (string | null);
    "newPassword": string;
    "logoutSessions": boolean;
};
export type CreateFrontendAccountRequest = (({
    "type": "withCredentials";
} & CreateCredentialsRequest) | ({
    "type": "withProvider";
} & {
    "provider": Provider;
    "providerId": string;
}));
export type SettingsValues = {

    /**
     * If true, anyone who knows of the server address will be able to
     * log in to the game server. Otherwise they must first be added to
     * the whitelist.
     */
    "publicServer": boolean;

    /**
     * If true, any user with an existing game account can log in to the
     * frontend. Otherwise an admin must first create a frontend account
     * for the user.
     */
    "allowFrontendLogin": boolean;
};
export type WhitelistEntry = {
    "providerId": string;
    "nameReference": (string | null);
};
export type Language = ("unknown" | "en-us" | "fr" | "es" | "de" | "it" | "nl" | "pt" | "zh-CHT" | "ko" | "ru" | "pl" | "da" | "fi" | "no" | "sv" | "ja" | "es-419" | "latampor" | "tr" | "ar" | "zh-CN" | "en-uk" | "el" | "cs" | "hu");
export type U32 = number;
export type ConstructionPointType = ("deliveryBase" | "preppersShelter" | "stageSafetyHouse" | "playerSafetyHouse" | "netSafetyHouse" | "stagePost" | "playerPost" | "netPost" | "stageWatchTower" | "playerWatchTower" | "netWatchTower" | "_Reserved0" | "_Reserved1" | "_Reserved2" | "stageCharger" | "playerCharger" | "netCharger" | "stageRainShelter" | "playerRainShelter" | "netRainShelter" | "mulePost" | "stageZipline" | "playerZipline" | "netZipline" | "stageLadder" | "playerLadder" | "netLadder" | "stageFieldRope" | "playerFieldRope" | "netFieldRope" | "stageBridge30m" | "playerBridge30m" | "netBridge30m" | "stageBridge45m" | "playerBridge45m" | "netBridge45m" | "roadRebuilder" | "_Reserved3" | "_Reserved4" | "_Reserved5" | "_Reserved6" | "_Reserved7" | "_Reserved8" | "_Reserved9" | "_Reserved10" | "_Reserved11");
export type Area = ("area00" | "area01" | "area02" | "area03" | "area04" | "warrior01" | "warrior02" | "warrior03" | "beach01" | "empty" | "frange01" | "nm01" | "nm02" | "nm04" | "_Reserved0" | "_Reserved1" | "_Reserved2" | "_Reserved3" | "_Reserved4" | "_Reserved5" | "_Reserved6" | "_Reserved7" | "_Reserved8" | "_Reserved9" | "a" | "b" | "c" | "d" | "e");
export type F64 = number;
export type QpidAreaMetaData = {
    "orderInList": U32;
    "constructionType": ConstructionPointType;
    "area": Area;
    "location": [F64, F64, F64];
};
export type QpidArea = {
    "qpidId": I32;
    "names": Record<Language, string>;
    "metadata": QpidAreaMetaData;
};
export type F32 = number;
export type QpidObjectType = ("unknown" | "ladder" | "climbingAnchor" | "bridge" | "timefallShelter" | "safeHouse" | "zipline" | "jumpRamp" | "chiralBridge" | "sign" | "powerGenerator" | "postbox" | "watchtower" | "restingStone" | "peeMushroom" | "motorbike" | "truck" | "cargoCatapult" | "cargoCatapultPod");
export type GameAccountSummary = {
    "id": string;
    "name": string;
};
export type QpidObject = {
    "id": string;
    "location": [F32, F32, F32];
    "locationId": I32;
    "objectType": QpidObjectType;
    "unknownType": ([string, string] | null);

    /**
     * Only applicable for vehicles. If true then the vehicle is not in a garage (i think).
     * For all other object types this is always true.
     */
    "isLost": boolean;
    "creator": GameAccountSummary;
};
export type ContentsType = ("commodity" | "weapon" | "equipment" | "special" | "rawMaterial");
export type Baggage = {
    "missionId": string;
    "id": I64;
    "name": string;
    "amount": I32;
    "location": [F32, F32, F32];
    "locationId": I32;
    "targetLocationId": (I32 | null);
    "targetLocationName": (string | null);
    "category": ContentsType;
    "isWasted": boolean;
    "isBroken": boolean;
    "creator": GameAccountSummary;
};
export type ObjectMetaData = {
    "uuid": string;
};
export type BaggageCaseType = ("normal" | "liquidOnly" | "weapon" | "item" | "equipment" | "bBPod" | "bodyBag" | "dummy" | "handcuffs" | "material" | "cart" | "constructionMachine" | "ladder" | "delicate" | "rope" | "vehicle" | "livingThing" | "smallDelicate" | "toxicGas");
export type ContentsDamageType = ("normal" | "fragile" | "delicate" | "danger" | "sensitiveToTimefall" | "equipment" | "livingThing" | "mustKeepHorizontally" | "cool");
export type VolumeType = ("small" | "medium" | "large" | "extraLarge" | "human");
export type BaggageMetaData = {
    "typeCase": BaggageCaseType;
    "typeContentsDamage": ContentsDamageType;
    "typeContents": ContentsType;
    "typeVolume": VolumeType;
    "amount": U32;
    "subAmount": U32;
    "weight": F32;
    "durabilityContents": U32;
    "durabilityCase": U32;
    "initialDurabilityContents": U32;
    "initialDurabilityCase": U32;
    "missionId": U32;
};
export type LocalizedBaggageData = {
    "nameHash": U32;
    "objectMetadata": ObjectMetaData;
    "baggageMetadata": BaggageMetaData;
    "name": string;
    "description": string;
};
export type BaggageAmount = {
    "nameHash": U32;
    "amount": I32;
};
export type BaggageWithLocationAndAmount = {
    "nameHash": U32;
    "amount": I32;
    "location": [F32, F32, F32];
};
export type EditMissionData = (({
    "type": "delivery";
} & {
    "startQpidId": I32;
    "endQpidId": I32;
    "baggageAmounts": (BaggageAmount)[];
    "rewardAmounts": (BaggageAmount)[];
}) | ({
    "type": "collection";
} & {
    "targetQpidId": I32;
    "baggageAmounts": (BaggageAmount)[];
    "rewardAmounts": (BaggageAmount)[];
}) | ({
    "type": "recovery";
} & {
    "targetQpidId": I32;
    "baggages": (BaggageWithLocationAndAmount)[];
    "rewardAmounts": (BaggageAmount)[];
}));
export type EditMissionRequest = {
    "missionId": (string | null);
    "data": EditMissionData;
};
