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

  /**
   * Use this if this button is the submit button in a form.
   * This will bind the onSubmit event instead of the button's click event.
   *
   * Note that this also calls preventDefault on the submit event.
   */
  isForm?: boolean;
}

function SaveButton<T>({ saveAction, isForm, children }: Props<T>) {
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const [waitingForCooldown, setWaitingForCooldown] = React.useState(false);
  let buttonRef = React.useRef<HTMLButtonElement>();

  React.useEffect(() => {
    if (buttonRef.current == null) {
      return;
    }

    const node = buttonRef.current;

    // Bind to form submit event if isForm = true, otherwise bind to click event
    if (isForm) {
      const form = node.closest("form");

      if (form != null) {
        const callback = (ev: SubmitEvent) => {
          ev.preventDefault();
          doSave();
        };

        form.addEventListener("submit", callback);

        return () => {
          form.removeEventListener("submit", callback);
        };
      } else {
        console.warn(
          "No <form> parent node found for SaveButton with isForm = true"
        );
      }
    } else {
      node.addEventListener("click", doSave);

      return () => {
        node.removeEventListener("click", doSave);
      };
    }
  }, [saveAction, isForm, buttonRef.current]);

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
      ref={(elm) => (buttonRef.current = elm)}
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
