import * as React from "react";
import { AgGridReact } from "ag-grid-react";
import {
  FrontendAccountListItem,
  FrontendPermissions,
} from "../../../api_types";
import { useState } from "react";
import {
  ColDef,
  ICellRendererParams,
  RowClickedEvent,
  ValueFormatterParams,
} from "ag-grid-community";
import Tag from "../../../components/tag";
import { permissionToReadableString } from "../../../services/accounts_service";
import { Link } from "react-router-dom";
import { Gear } from "@phosphor-icons/react";
import { formatPermissions } from "../../../utils/table_value_formatters/permissions";
import { TableButtons, TableWrapper } from "./table_base";

interface Props {
  accounts: FrontendAccountListItem[] | null | undefined;
}

const Permissions = (
  props: ICellRendererParams<FrontendAccountListItem, FrontendPermissions[]>
) => {
  return props.value?.map((permission) => (
    <Tag key={permission}>{permissionToReadableString(permission)}</Tag>
  ));
};

const Buttons = (props: ICellRendererParams<FrontendAccountListItem>) => {
  if (props.data == null) {
    return null;
  }

  return (
    <TableButtons>
      <Link to={`frontend/${props.data.id}`} title="Edit">
        <Gear weight="regular" />
      </Link>
    </TableButtons>
  );
};

const FrontendAccountsTable = ({ accounts }: Props) => {
  const [defaultColDef] = useState<ColDef>({
    sortable: true,
    filter: true,
    resizable: false,
    flex: 1,
  });

  const [columnDefs] = useState<ColDef[]>([
    { field: "name" },
    { field: "gameId" },
    {
      field: "permissions",
      cellRenderer: Permissions,
      valueFormatter: formatPermissions,
    },
    {
      cellRenderer: Buttons,
      maxWidth: 55,
      filter: false,
      sortable: false,
      suppressMovable: true,
    },
  ]);

  return (
    <TableWrapper>
      <AgGridReact
        columnDefs={columnDefs}
        defaultColDef={defaultColDef}
        rowData={accounts}
        domLayout="autoHeight"
        pagination={true}
        rowSelection="single"
        suppressCellFocus={true}
        suppressRowClickSelection={true}
        enableCellTextSelection={true}
      />
    </TableWrapper>
  );
};

export default FrontendAccountsTable;
