import { AgGridReact } from "ag-grid-react";
import { LocalizedBaggageData } from "../../api_types";
import BaggageSelector from "./baggage_selector";
import styled from "styled-components";
import {
  ColDef,
  NewValueParams,
  ValueFormatterParams,
} from "ag-grid-community";

const Wrapper = styled.div`
  & .ag-theme-alpine-dark {
    height: 250px;
  }
`;

export interface SelectedCargo {
  cargo: LocalizedBaggageData;
  amount: number;
}

interface Props {
  values: SelectedCargo[];
  onChange: (values: SelectedCargo[]) => void;
  baggages: LocalizedBaggageData[];
}

const defaultColDef: ColDef = {
  sortable: true,
  filter: true,
  resizable: true,
  flex: 1,
};

const CargoAmountSelector = ({ values, onChange, baggages }: Props) => {
  function onAddBaggage(baggage: LocalizedBaggageData) {
    if (values.some((v) => v.cargo.nameHash === baggage.nameHash)) {
      alert(
        "Baggage is already added. Edit the amount if you want to add more of the same cargo."
      );
      return;
    }

    onChange([
      ...values,
      {
        cargo: baggage,
        amount: 1,
      },
    ]);
  }

  function onClickClear() {
    if (confirm("Are you sure you want to remove all cargo?")) {
      onChange([]);
    }
  }

  const columnDefs: ColDef[] = [
    {
      field: "cargo.name",
      headerName: "Name",
    },
    {
      field: "amount",
      editable: true,
      singleClickEdit: true,

      // Handle change event for when the amount is changed
      onCellValueChanged: (params: NewValueParams<SelectedCargo, number>) => {
        // Check if new value is invalid, confirm if it should be removed, if not change back to old value
        if (params.newValue == null || params.newValue === 0) {
          if (
            confirm(
              "The new amount is not valid, do you want to remove the cargo?"
            )
          ) {
            onChange(
              values.filter(
                (v) => v.cargo.nameHash !== params.data.cargo.nameHash
              )
            );
          } else {
            setTimeout(
              () => params.node.setDataValue("amount", params.oldValue),
              0
            );
          }

          return;
        }

        // Round new value to nearest integer
        let newValue = Math.round(params.newValue);

        if (newValue === 0) {
          // remove cargo
          onChange(
            values.filter(
              (v) => v.cargo.nameHash !== params.data.cargo.nameHash
            )
          );
        } else {
          onChange(
            values.map((v) => {
              if (v.cargo.nameHash === params.data.cargo.nameHash) {
                return {
                  ...v,
                  amount: newValue,
                };
              } else {
                return v;
              }
            })
          );
        }
      },
    },
    {
      field: "cargo.baggageMetadata.typeVolume",
      headerName: "Size",
    },
    {
      field: "cargo.baggageMetadata.weight",
      headerName: "Weight (kg)",
      valueFormatter: (params: ValueFormatterParams<SelectedCargo>) => {
        return `${
          params.data?.amount * params.data?.cargo.baggageMetadata.weight ?? 0
        } kg`;
      },
    },
  ];

  return (
    <Wrapper>
      <div className="ag-theme-alpine-dark">
        <AgGridReact
          columnDefs={columnDefs}
          defaultColDef={defaultColDef}
          rowData={values}
          enableCellChangeFlash
          suppressCellFocus
        />
      </div>
      <div className="buttons">
        <BaggageSelector
          baggages={baggages}
          value={null}
          text="Add"
          onChange={onAddBaggage}
        />
        <button
          type="button"
          className="warning"
          onClick={onClickClear}
          disabled={values.length === 0}
        >
          Clear
        </button>
      </div>
    </Wrapper>
  );
};

export default CargoAmountSelector;
