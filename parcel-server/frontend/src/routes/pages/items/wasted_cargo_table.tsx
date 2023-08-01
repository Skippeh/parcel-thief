import * as React from "react";
import { AgGridReact } from "ag-grid-react";
import { useState } from "react";
import { ColDef } from "ag-grid-community";
import { WastedCargoListItem } from "../../../api_types";

interface Props {
  items?: WastedCargoListItem[] | null;
}

const WastedCargoTable = ({ items }: Props) => {
  const [defaultColDef] = useState<ColDef>({
    sortable: true,
    filter: true,
    resizable: true,
    flex: 1,
  });

  const [columnDefs] = useState<ColDef[]>([
    { field: "name" },
    { field: "category" },
    { field: "broken" },
    { field: "location" },
    { field: "creator", headerName: "Player" },
  ]);

  return (
    <div className="ag-theme-alpine-dark">
      <AgGridReact
        columnDefs={columnDefs}
        defaultColDef={defaultColDef}
        rowData={items}
        domLayout="autoHeight"
        enableCellTextSelection={true}
        pagination={true}
      />
    </div>
  );
};

export default WastedCargoTable;
