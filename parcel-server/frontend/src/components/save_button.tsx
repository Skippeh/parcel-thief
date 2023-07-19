import * as React from "react";
import { ApiResponse } from "../services";
import { Submit as FormSubmit } from "./form";
import { CheckFat, FloppyDisk, Icon, Spinner, X } from "@phosphor-icons/react";
import styled from "styled-components";

const Submit = styled(FormSubmit)`
  & .icon {
    margin-right: 0.2rem;
    vertical-align: middle;
  }
`;

interface Props<T> extends React.PropsWithChildren {
  saveAction: () => Promise<ApiResponse<T>>;
}

function SaveButton<T>({ saveAction, children }: Props<T>) {
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const [waitingForCooldown, setWaitingForCooldown] = React.useState(false);

  const doSave = async () => {
    setLoading(true);
    setError(null);

    const response = await saveAction();

    if (response.error != null) {
      setError(response.error);
    } else {
      setError(null);
    }

    setLoading(false);
    setWaitingForCooldown(true);

    setTimeout(() => {
      setWaitingForCooldown(false);
    }, 2000);
  };

  return (
    <Submit
      onClick={doSave}
      disabled={loading || waitingForCooldown}
      title={error || ""}
    >
      <span className="icon">
        {loading && <Spinner weight="bold" className="spin" />}
        {waitingForCooldown && !error && <CheckFat weight="fill" />}
        {error && <X weight="bold" />}
        {!waitingForCooldown && !loading && !error && (
          <FloppyDisk weight="fill" />
        )}
      </span>
      {children}
    </Submit>
  );
}

export default SaveButton;
