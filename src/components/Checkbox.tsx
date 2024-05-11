import { useState } from 'preact/hooks'
import { JSX } from 'preact/jsx-runtime'

import './Checkbox.css'

interface Props {
  id: string
  initialChecked?: boolean
  label?: string | JSX.Element
  onChange?: (checked: boolean) => void
}

export function Checkbox(props: Props) {
  const [checked, setChecked] = useState(props.initialChecked || false)

  return (
    <div class="checkbox">
      <input
        type="checkbox"
        id={props.id}
        checked={checked}
        onChange={() => {
          setChecked(!checked)
          props.onChange && props.onChange(!checked)
        }}
      />

      <label class="checkbox-box" for={props.id}>
        {
          checked && (
            <img src="check.svg" alt="Checked" />
          )
        }
      </label>
      
      {
        props.label && (
          <label for={props.id}>{props.label}</label>
        )
      }
    </div>
  )
}