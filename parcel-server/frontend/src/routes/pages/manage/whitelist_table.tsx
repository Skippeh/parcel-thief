import * as React from "react";
import { AgGridReact } from "ag-grid-react";
import { WhitelistEntry } from "../../../api_types";
import {
  CellEditRequestEvent,
  ColDef,
  RowEditingStartedEvent,
  RowEditingStoppedEvent,
} from "ag-grid-community";
import { styled } from "styled-components";

interface Props {
  whitelist: WhitelistEntry[] | null | undefined;
  setWhitelist: (whitelist: WhitelistEntry[]) => void;
}

const Wrapper = styled.div.attrs({
  className: "ag-theme-alpine-dark",
})`
  margin-top: 1.5rem;
`;

const WhitelistTable = ({ whitelist, setWhitelist }: Props) => {
  const [rows, setRows] = React.useState<WhitelistEntry[]>([]);

  const defaultColDef: ColDef = {
    sortable: true,
    filter: true,
    flex: 1,
    editable: true,
    singleClickEdit: true,
  };

  const colDefs: ColDef[] = [
    {
      field: "providerId",
    },
    {
      field: "nameReference",
      headerName: "Name",
    },
  ];

  React.useEffect(() => {
    setRows([...whitelist, { nameReference: "", providerId: "" }]);
  }, [whitelist]);

  function onRowEditingStopped(
    ev: RowEditingStoppedEvent<WhitelistEntry, WhitelistEntry[]>
  ) {
    const rowId = ev.node.id;
    const newData = ev.data;

    if (!newData.providerId?.length && newData.nameReference?.length) {
      alert("The provider id is required");
    }

    const newRows = rows
      .map((entry, index) => {
        if (index.toString() == rowId) {
          return newData;
        }

        return entry;
      })
      .filter((row) => {
        const providerId = row.providerId?.trim();
        const nameReference = row.nameReference?.trim();

        return (
          providerId?.length || (providerId?.length && nameReference?.length)
        );
      });

    setWhitelist(newRows);
  }

  return (
    <Wrapper>
      <AgGridReact
        defaultColDef={defaultColDef}
        columnDefs={colDefs}
        rowData={rows}
        domLayout="autoHeight"
        pagination
        enableCellTextSelection
        editType="fullRow"
        stopEditingWhenCellsLoseFocus
        onRowEditingStopped={onRowEditingStopped}
      />
    </Wrapper>
  );
};

export default WhitelistTable;
