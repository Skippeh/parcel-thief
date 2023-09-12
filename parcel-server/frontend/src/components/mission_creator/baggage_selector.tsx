import styled from "styled-components";
import { LocalizedBaggageData } from "../../api_types";
import * as Dialog from "../dialog";
import * as Colors from "@radix-ui/colors";
import { X } from "@phosphor-icons/react";
import { AgGridReact } from "ag-grid-react";
import {
  ColDef,
  GetRowIdParams,
  GridReadyEvent,
  RowDoubleClickedEvent,
  SelectionChangedEvent,
} from "ag-grid-community";
import { useState } from "react";

const DialogTrigger = styled(Dialog.Trigger)`
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0;
  margin: 0.25rem 0;
  background: ${Colors.grayDark.gray4};
  border: 1px solid ${Colors.whiteA.whiteA11};
  border-radius: 2px;

  &:hover,
  &:focus-visible {
    background: ${Colors.grayDark.gray3};
    border: 1px solid ${Colors.whiteA.whiteA10};
  }

  &:disabled {
    &,
    &:hover,
    &:focus {
      background: ${Colors.grayDark.gray5};
      border: 1px solid ${Colors.whiteA.whiteA9};
    }
  }

  // clear button styling
  & button {
    margin: 0;
    border-radius: 0;
    background: ${Colors.whiteA.whiteA11};
    color: black;

    &:hover,
    &:focus-visible {
      background: ${Colors.whiteA.whiteA10};
      color: black;
    }
  }

  & .selectedValue {
    padding: 0.6rem 0.9rem;
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }
`;
const DialogOverlay = styled(Dialog.Overlay)`
  z-index: 2000;
`;
const DialogContent = styled(Dialog.Content)`
  z-index: 2000;

  & .ag-theme-alpine-dark {
    width: 95vw;
    max-width: 1000px;
    height: 300px;
  }

  // 'Select' button styling
  & button {
    margin-left: 0;
    margin-bottom: 0;
    margin-top: 1.5rem;
  }
`;

interface Props {
  baggages: LocalizedBaggageData[];
  value: LocalizedBaggageData | null;
  onChange?: (value: LocalizedBaggageData | null) => void;
  onValueCleared?: () => void;
  disabled?: boolean;
}

const BaggageSelector = ({
  baggages,
  value,
  disabled,
  onChange,
  onValueCleared,
}: Props) => {
  const [selectedRows, setSelectedRows] = useState<LocalizedBaggageData[]>([]);
  const [modalOpen, setModalOpen] = useState(false);

  const defaultColDef: ColDef = {
    sortable: true,
    filter: true,
    resizable: true,
    flex: 1,
  };
  const columnDefs: ColDef[] = [
    {
      field: "name",
    },
    {
      field: "baggageMetadata.typeVolume",
      headerName: "Size",
    },
    {
      field: "baggageMetadata.weight",
      headerName: "Weight (kg)",
    },
    {
      field: "baggageMetadata.typeContentsDamage",
      headerName: "Damage type",
    },
  ];

  function onGridReady(ev: GridReadyEvent<LocalizedBaggageData>) {
    // Select the corresponding node if value is set
    if (value != null) {
      const nodes = [ev.api.getRowNode(value.nameHash.toString())];
      ev.api.setNodesSelected({
        nodes,
        newValue: true,
      });
    }
  }

  function onSelectionChanged(ev: SelectionChangedEvent<LocalizedBaggageData>) {
    setSelectedRows(ev.api.getSelectedRows());
  }

  function onSelectClicked() {
    if (!selectedRows.length) {
      return;
    }

    onChange(selectedRows[0]);
    setModalOpen(false);
  }

  function onRowDoubleClicked(ev: RowDoubleClickedEvent<LocalizedBaggageData>) {
    onChange(ev.data);
    setModalOpen(false);
  }

  return (
    <Dialog.Root open={modalOpen} onOpenChange={(open) => setModalOpen(open)}>
      <DialogTrigger disabled={disabled}>
        <div className="selectedValue">
          {value != null
            ? `${value.name} (${value.baggageMetadata.typeVolume}, ${value.baggageMetadata.weight} kg)`
            : "Select Baggage"}
        </div>
        {value && !disabled && (
          <button
            type="button"
            onClick={(ev) => {
              ev.stopPropagation(); // prevent parent button from triggering
              onChange(null);

              if (onValueCleared) {
                onValueCleared();
              }
            }}
            title="Clear value"
          >
            <X weight="bold" />
          </button>
        )}
      </DialogTrigger>
      <Dialog.Portal>
        <DialogOverlay />
        <DialogContent>
          <Dialog.Title>Select Baggage</Dialog.Title>
          <div className="ag-theme-alpine-dark">
            <AgGridReact
              columnDefs={columnDefs}
              rowData={baggages}
              defaultColDef={defaultColDef}
              rowSelection="single"
              suppressCellFocus
              onSelectionChanged={onSelectionChanged}
              onGridReady={onGridReady}
              getRowId={(row: GetRowIdParams<LocalizedBaggageData>) =>
                row.data.nameHash.toString()
              }
              onRowDoubleClicked={onRowDoubleClicked}
            />
          </div>
          <button
            type="button"
            disabled={!selectedRows.length}
            onClick={onSelectClicked}
          >
            Select
          </button>
        </DialogContent>
      </Dialog.Portal>
    </Dialog.Root>
  );
};

export default BaggageSelector;
