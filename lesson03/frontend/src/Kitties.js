import React, { useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { TxButton } from './substrate-lib/components'

export default function Main (props) {

  const [kittyCnt] = useState(0)

  const [status] = useState('')

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          label={kittyCnt} type='SIGNED-TX'
          attrs={{
            palletRpc: 'kitties',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>

}
