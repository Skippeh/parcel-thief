import { ColDef, ICellRendererParams } from "ag-grid-community";
import { AgGridReact } from "ag-grid-react";
import * as React from "react";
import { useState } from "react";
import { styled } from "styled-components";
import { GameAccountListItem } from "../../../api_types";
import { formatDate } from "../../../utils/table_value_formatters/date";

const Wrapper = styled.div``;

const Buttons = (props: ICellRendererParams<GameAccountListItem>) => {
  if (props.data == null) {
    return null;
  }

  return <div></div>;
};

interface Props {
  accounts: GameAccountListItem[] | null | undefined;
}

const GameAccountsTable = ({ accounts }: Props) => {
  const [defaultColDef] = useState<ColDef>({
    sortable: true,
    filter: true,
    resizable: false,
    flex: 1,
  });

  const [columnDefs] = useState<ColDef[]>([
    { field: "name" },
    { field: "gameId" },
    { field: "provider" },
    { field: "lastLogin", valueFormatter: formatDate },
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

export default GameAccountsTable;
