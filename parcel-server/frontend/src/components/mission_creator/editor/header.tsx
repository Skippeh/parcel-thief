import styled from "styled-components";
import { EditMissionData } from "../../../api_types";
import * as Colors from "@radix-ui/colors";
import { useWizard } from "../../wizard";
import { PropsWithChildren } from "react";

const Wrapper = styled.div`
  width: 250px;
  height: 100%;
  border-right: 1px solid ${Colors.grayDark.gray11};

  & > h3 {
    margin-top: 0;
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
  }

  & ul {
    list-style: none;
    padding: 0;
    margin: 0;
    margin-left: 0.2rem;

    & li {
      &:not(:last-child) {
        margin-bottom: 0.2rem;
      }

      &.active {
        font-weight: bold;
      }
    }
  }
`;

interface StepProps extends PropsWithChildren {
  step: number;
  disabled?: boolean;
  title?: string;
}

const Step = ({ step, disabled, title, children }: StepProps) => {
  const { goToStep, activeStep } = useWizard();

  return (
    <li className={activeStep === step ? "active" : ""}>
      <a
        className={disabled && "disabled"}
        onClick={() => !disabled && goToStep(step)}
        title={title}
      >
        {children}
      </a>
    </li>
  );
};

interface HeaderProps {
  data: EditMissionData;
}

const Header = ({ data }: HeaderProps) => {
  function renderSteps() {
    switch (data?.type) {
      case "delivery": {
        return (
          <>
            <Step step={1}>Pickup location</Step>
            <Step step={2}>Dropoff location</Step>
            <Step
              step={3}
              disabled={data.endQpidId == -1}
              title={data.endQpidId == -1 && "Choose dropoff location first"}
            >
              Cargo
            </Step>
            <Step step={4}>Reward</Step>
          </>
        );
      }
      case "collection": {
        return <></>;
      }
      case "recovery": {
        return <></>;
      }
      default:
        return null;
    }
  }

  return (
    <Wrapper>
      <h3>Steps</h3>
      <ul>
        <Step step={0}>Mission type</Step>
        {renderSteps()}
      </ul>
    </Wrapper>
  );
};

export default Header;
