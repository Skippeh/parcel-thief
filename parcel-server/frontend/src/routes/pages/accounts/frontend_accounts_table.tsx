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
import { UserGear } from "@phosphor-icons/react";
import styled from "styled-components";
import * as Colors from "@radix-ui/colors";

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

const formatPermissions = (
  params: ValueFormatterParams<FrontendAccountListItem, FrontendPermissions[]>
): string => {
  return (
    params.value
      ?.map((permission) => permissionToReadableString(permission))
      .join(", ") ?? ""
  );
};

const Wrapper = styled.div`
  & .ag-row .buttons {
    opacity: 0.5;
    transition: opacity 0.1s ease-out;
  }

  & .ag-row:hover .buttons {
    opacity: 1;
  }
`;

const ButtonsWrapper = styled.div`
  & a {
    color: ${Colors.whiteA.whiteA12};
    font-size: 1.3rem;
    vertical-align: middle;
  }
`;

const Buttons = (props: ICellRendererParams<FrontendAccountListItem>) => {
  if (props.data == null) {
    return null;
  }

  return (
    <ButtonsWrapper className="buttons">
      <Link to={`frontend/${props.data.id}`} title="Edit">
        <UserGear weight="fill" />
      </Link>
    </ButtonsWrapper>
  );
};

const FrontendAccountsTable = ({ accounts }: Props) => {
  const [defaultColDef] = useState<ColDef>({
    sortable: true,
    filter: true,
    resizable: false,
    flex: 2,
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
    <Wrapper className="ag-theme-alpine-dark">
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
    </Wrapper>
  );
};

export default FrontendAccountsTable;
