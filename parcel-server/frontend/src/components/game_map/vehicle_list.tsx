import { AgGridReact } from "ag-grid-react";
import { QpidObject } from "../../api_types";
import { DefaultProps, TableWrapper } from "./table_common";
import { useState } from "react";
import { ColDef } from "ag-grid-community";

interface Props {
  vehicles: QpidObject[];
}

const VehicleList = ({ vehicles }: Props) => {
  const [columnDefs] = useState<ColDef[]>([
    {
      field: "objectType",
      headerName: "Type",
    },
    {
      field: "creator.name",
      headerName: "Owner",
    },
  ]);

  return (
    <TableWrapper>
      <AgGridReact
        rowData={vehicles}
        columnDefs={columnDefs}
        {...DefaultProps}
      />
    </TableWrapper>
  );
};

export default VehicleList;
