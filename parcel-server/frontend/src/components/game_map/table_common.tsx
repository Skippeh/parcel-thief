import { ColDef } from "ag-grid-community";
import { AgGridReactProps } from "ag-grid-react/lib/shared/interfaces";
import styled from "styled-components";

export const DefaultColDef: ColDef = {
  sortable: true,
  filter: true,
  resizable: true,
  flex: 1,
};

export const DefaultProps: AgGridReactProps = {
  rowSelection: "single",
  suppressCellFocus: true,
  suppressRowClickSelection: true,
  enableCellTextSelection: true,
  defaultColDef: DefaultColDef,
};

export const TableWrapper = styled.div.attrs({
  className: "ag-theme-alpine-dark",
})`
  --ag-header-height: 35px;
  --ag-row-height: 35px;
  --ag-font-size: 12px;
  height: 250px;
`;
