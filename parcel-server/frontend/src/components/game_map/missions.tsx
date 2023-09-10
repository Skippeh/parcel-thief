import styled from "styled-components";
import * as Dialog from "../dialog";
import MissionEditor from "../mission_creator/editor";
import * as Colors from "@radix-ui/colors";
import { Area } from "../../api_types";

const DialogOverlay = styled(Dialog.Overlay)`
  // Use more alpha than parent because of the map being very "messy" in the background
  background: ${Colors.blackA.blackA11};
  z-index: 2000;
`;

const DialogContent = styled(Dialog.Content)`
  max-width: 95%;
  width: 1000px;
  z-index: 2000;
`;

interface Props {
  area: Area;
  qpidId: number;
}

const Missions = ({ area, qpidId }: Props) => {
  return (
    <Dialog.Root>
      <Dialog.Trigger>Create mission</Dialog.Trigger>
      <Dialog.Portal>
        <DialogOverlay />
        <DialogContent>
          <Dialog.Title>Create new mission</Dialog.Title>
          <MissionEditor area={area} startQpidId={qpidId} />
        </DialogContent>
      </Dialog.Portal>
    </Dialog.Root>
  );
};

export default Missions;
