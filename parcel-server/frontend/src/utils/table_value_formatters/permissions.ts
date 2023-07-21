import { ValueFormatterParams } from "ag-grid-community";
import { FrontendPermissions } from "../../api_types";
import { permissionToReadableString } from "../../services/accounts_service";

export function formatPermissions<
  TData,
  TValue extends FrontendPermissions[] | null | undefined
>(params: ValueFormatterParams<TData, TValue>): string {
  return (
    params.value
      ?.map((permission) => permissionToReadableString(permission))
      .join(", ") ?? ""
  );
}
