import { AgGridReact } from "ag-grid-react";
import { Baggage } from "../../api_types";
import { useState } from "react";
import { ColDef } from "ag-grid-community";
import { DefaultProps, TableWrapper } from "./table_common";

interface Props {
  baggages: Baggage[];
}

const SharedCargoList = ({ baggages }: Props) => {
  const [columnDefs] = useState<ColDef[]>([
    {
      field: "name",
    },
    {
      field: "amount",
      flex: 0,
      width: 100,
    },
    {
      field: "category",
    },
    {
      field: "creator.name",
      headerName: "Donator",
    },
  ]);

  return (
    <TableWrapper>
      <AgGridReact
        rowData={baggages}
        columnDefs={columnDefs}
        {...DefaultProps}
      />
    </TableWrapper>
  );
};

export default SharedCargoList;
