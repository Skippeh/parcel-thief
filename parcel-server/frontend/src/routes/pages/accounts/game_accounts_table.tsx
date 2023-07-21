import { ColDef, ICellRendererParams } from "ag-grid-community";
import { AgGridReact } from "ag-grid-react";
import * as React from "react";
import { useState } from "react";
import { GameAccountListItem } from "../../../api_types";
import { formatDate } from "../../../utils/table_value_formatters/date";
import { TableButtons, TableWrapper } from "./table_base";
import { Link, useNavigate } from "react-router-dom";
import { Gear, Plus } from "@phosphor-icons/react";
import * as Dialog from "../../../components/dialog";
import SaveButton, { CooldownDelay } from "../../../components/save_button";
import { ApiResponse } from "../../../services";
import { createFrontendAccount } from "../../../services/accounts_service";

const Buttons = (props: ICellRendererParams<GameAccountListItem>) => {
  const [open, setOpen] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  if (props.data == null) {
    return null;
  }

  async function onCreateLocalAccount(): Promise<ApiResponse<number>> {
    setError(null);
    const response = await createFrontendAccount({
      type: "withProvider",
      provider: props.data.provider,
      providerId: props.data.providerId,
    });

    if (response.data != null) {
      setTimeout(() => {
        navigate(`/frontend/${response.data}`);
      }, CooldownDelay);
    } else {
      setError(response.error);
    }

    return response;
  }

  function onOpenChange(open: boolean) {
    setOpen(open);

    if (!open) {
      setError(null);
    }
  }

  return (
    <TableButtons>
      {props.data.frontendId && (
        <Link
          to={`frontend/${props.data.frontendId}`}
          title="View frontend account"
        >
          <Gear weight="regular" />
        </Link>
      )}
      {props.data.frontendId == null && (
        <Dialog.Root onOpenChange={onOpenChange}>
          <Dialog.Trigger asChild>
            <a title="Create frontend account">
              <Plus weight="regular" />
            </a>
          </Dialog.Trigger>
          <Dialog.Portal>
            <Dialog.Overlay />
            <Dialog.Content>
              <Dialog.Title>Create frontend account</Dialog.Title>
              <Dialog.Buttons>
                <p>
                  Are you sure you want to allow{" "}
                  <strong>{props.data.name}</strong> to log in to the frontend?
                  By default no permissions are given.
                </p>
                <div>
                  <span className="error">{error}</span>
                </div>
                <SaveButton saveAction={onCreateLocalAccount}>Yes</SaveButton>
                <Dialog.Close className="secondary">No</Dialog.Close>
              </Dialog.Buttons>
            </Dialog.Content>
          </Dialog.Portal>
        </Dialog.Root>
      )}
    </TableButtons>
  );
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
    <TableWrapper>
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
    </TableWrapper>
  );
};

export default GameAccountsTable;
