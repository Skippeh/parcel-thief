import { ValueFormatterParams } from "ag-grid-community";

export function formatDate<TData, TValue extends string | null | undefined>(
  params: ValueFormatterParams<TData, TValue>
) {
  if (params.value == null) {
    return "";
  }

  const date = new Date(params.value);
  return date.toLocaleString();
}
