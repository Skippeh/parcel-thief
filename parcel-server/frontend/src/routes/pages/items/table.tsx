import * as React from "react";
import { AgGridReact } from "ag-grid-react";
import { useState } from "react";
import { ColDef, GridReadyEvent } from "ag-grid-community";
import { BaggageListItem } from "../../../api_types";

interface Props {
  items?: BaggageListItem[] | null;
}

const Table = ({ items }: Props) => {
  const [defaultColDef] = useState<ColDef>({
    sortable: true,
    filter: true,
    resizable: true,
  });

  const [columnDefs] = useState<ColDef[]>([
    { field: "name" },
    { field: "amount" },
    { field: "category" },
    { field: "location" },
    { field: "creator", headerName: "Donator" },
  ]);

  function onGridReady(ev: GridReadyEvent) {
    ev.api.sizeColumnsToFit();
  }

  return (
    <div className="ag-theme-alpine-dark">
      <AgGridReact
        columnDefs={columnDefs}
        defaultColDef={defaultColDef}
        rowData={items}
        domLayout="autoHeight"
        onGridReady={onGridReady}
        enableCellTextSelection={true}
        pagination={true}
      />
    </div>
  );
};

export default Table;
