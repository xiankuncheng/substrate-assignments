import React, { useEffect, useState } from 'react';
import { Form, Grid } from 'semantic-ui-react';

import { useSubstrateState } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

import KittyCards from './KittyCards';

export default function Kitties(props) {
  const { api, keyring, currentAccount } = useSubstrateState();

  const [kittyIndexes, setKittyIndexes] = useState([]);
  const [kitties, setKitties] = useState([]);
  const [status, setStatus] = useState('');

  useEffect(() => {
    const fetchKittyIndexes = async () => {
      const kittyIndex = (
        await api.query.kittiesModule.nextKittyId()
      ).toNumber();
      if (kittyIndex <= 0) {
        return;
      }
      setKittyIndexes(Array.from(Array(kittyIndex).keys()));
    };

    fetchKittyIndexes();
  }, [api, status, keyring, setKittyIndexes]);

  useEffect(() => {
    let unsub = null;

    const fetchKitties = async () => {
      const owners = await api.query.kittiesModule.kittyOwner.multi(
        kittyIndexes
      );
      const kittyDNAs = await api.query.kittiesModule.kitties.multi(
        kittyIndexes
      );
      const kitties = kittyIndexes.map(kittyIndex => ({
        id: kittyIndex,
        dna: kittyDNAs[kittyIndex].value,
        owner: owners[kittyIndex].value.toJSON(),
      }));
      setKitties(kitties);
    };

    fetchKitties();

    return () => {
      unsub && unsub();
    };
  }, [api, keyring, kittyIndexes, setKitties]);

  return (
    <Grid.Column width={16}>
      <h1>小毛孩</h1>
      <KittyCards
        kitties={kitties}
        accountPair={currentAccount}
        setStatus={setStatus}
      />
      <Form style={{ margin: '1em 0' }}>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={currentAccount}
            label="创建小毛孩"
            type="SIGNED-TX"
            setStatus={setStatus}
            attrs={{
              palletRpc: 'kittiesModule',
              callable: 'create',
              inputParams: [],
              paramFields: [],
            }}
          />
        </Form.Field>
      </Form>
      <div style={{ overflowWrap: 'break-word' }}>{status}</div>
    </Grid.Column>
  );
}
