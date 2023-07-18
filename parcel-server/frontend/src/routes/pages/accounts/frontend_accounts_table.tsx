import * as React from "react";
import { AgGridReact } from "ag-grid-react";
import {
  FrontendAccountListItem,
  FrontendPermissions,
} from "../../../api_types";
import { useState } from "react";
import { ColDef, ICellRendererParams } from "ag-grid-community";
import Tag from "../../../components/tag";
import { permissionToReadableString } from "../../../services/accounts_service";

interface Props {
  accounts: FrontendAccountListItem[] | null | undefined;
}

const Permissions = (
  props: ICellRendererParams<FrontendAccountListItem, FrontendPermissions[]>
) => {
  return props.value?.map((permission) => (
    <Tag>{permissionToReadableString(permission)}</Tag>
  ));
};

const FrontendAccountsTable = ({ accounts }: Props) => {
  const [defaultColDef] = useState<ColDef>({
    sortable: true,
    filter: true,
    resizable: true,
    flex: 1,
  });

  const [columnDefs] = useState<ColDef[]>([
    { field: "name" },
    { field: "gameId" },
    { field: "permissions", cellRenderer: Permissions },
  ]);

  return (
    <div className="ag-theme-alpine-dark">
      <AgGridReact
        columnDefs={columnDefs}
        defaultColDef={defaultColDef}
        rowData={accounts}
        domLayout="autoHeight"
        pagination={true}
        rowSelection="single"
        suppressCellFocus={true}
        enableCellTextSelection={true}
      />
    </div>
  );
};

export default FrontendAccountsTable;
