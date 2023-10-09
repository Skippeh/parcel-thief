import SelectComponent, {
  GroupBase,
  StylesConfig,
  ThemeConfig,
} from "react-select";
import { StateManagerProps } from "react-select/dist/declarations/src/useStateManager";
import { RefAttributes } from "react";
import Select from "react-select/dist/declarations/src/Select";
import * as Colors from "@radix-ui/colors";

type Props<
  Option,
  IsMulti extends boolean = false,
  Group extends GroupBase<Option> = GroupBase<Option>
> = StateManagerProps<Option, IsMulti, Group> &
  RefAttributes<Select<Option, IsMulti, Group>>;

const styles: StylesConfig = {
  menuList: (base, _state) => ({
    ...base,
    paddingTop: 0,
    paddingBottom: 0,
    border: `1px solid ${Colors.grayDark.gray11}`,
    borderRadius: "2px",
  }),
  option: (base, state) => ({
    ...base,
    backgroundColor: state.isSelected
      ? state.isFocused
        ? Colors.indigoDark.indigo9
        : Colors.blueDark.blue8
      : state.isFocused
      ? Colors.grayDark.gray7
      : Colors.grayDark.gray4,
  }),
  control: (base) => ({
    ...base,
    backgroundColor: Colors.grayDark.gray4,
    borderRadius: "2px",
  }),
  placeholder: (base) => ({
    ...base,
    color: Colors.grayDark.gray11,
  }),
  singleValue: (base) => ({
    ...base,
    color: Colors.grayDark.gray12,
  }),
};

export default <
  Option = unknown,
  IsMulti extends boolean = false,
  Group extends GroupBase<Option> = GroupBase<Option>
>(
  props: Props<Option, IsMulti, Group>
) => {
  return <SelectComponent {...props} styles={styles} />;
};
