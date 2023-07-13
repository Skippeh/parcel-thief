import "ag-grid-community/styles/ag-grid.css";
import "ag-grid-community/styles/ag-theme-alpine.css";

import * as React from "react";
import { AgGridReact } from "ag-grid-react";
import { useState } from "react";
import { ColDef, GridReadyEvent } from "ag-grid-community";

interface Props {}

const Table = ({}: Props) => {
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
    { field: "creator" },
  ]);

  const [rowData] = useState([
    {
      name: "Item 1",
      amount: 1,
      category: "Category 1",
      location: "Location 1",
      creator: "Creator 1",
    },
  ]);

  function onGridReady(ev: GridReadyEvent) {
    ev.api.sizeColumnsToFit();
  }

  return (
    <div className="ag-theme-alpine-dark">
      <AgGridReact
        columnDefs={columnDefs}
        defaultColDef={defaultColDef}
        rowData={rowData}
        domLayout="autoHeight"
        onGridReady={onGridReady}
      />
    </div>
  );
};

export default Table;
