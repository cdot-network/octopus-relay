import React, { useCallback, useState, useEffect } from "react";

import { Button, Modal, Form, Input, Result } from "antd";
import Big from 'big.js';

import TokenBadge from "../../components/TokenBadge";

function StakeModal({ visible, appchainId, onCancel, onOk }): React.ReactElement {
  const [isSubmiting, setIsSubmiting] = useState<boolean>(false);

  const [checkingAllowance, setCheckingAllowance] = useState<boolean>(false);
  const [needIncrementsAllowance, setNeedIncrementsAllowance] = useState(false);

  const onFinish = useCallback((values) => {
    setIsSubmiting(true);
    onOk(values);
  }, []);

  useEffect(() => {
    if (visible) {
      setCheckingAllowance(true);
      window.tokenContract.get_allowance({
        owner_id: window.accountId,
        escrow_account_id: window.contractName
      }).then((allowance) => {
        setCheckingAllowance(false);
        if (allowance == 0) {
          setNeedIncrementsAllowance(true);
        }
      });
    }
  }, [visible]);

  const incrementsAllowance = function() {
    window.tokenContract.inc_allowance(
      {
        escrow_account_id: window.contractName,
        amount: "999999999999999999999"
      },
      Big(1).times(10 ** 14).toFixed(),
      Big(3).times(10 ** 22).toFixed(),
    );
  };

  return (
    <Modal visible={visible} title="Stake" 
      onCancel={onCancel} destroyOnClose={true} footer={null}>
      {
        needIncrementsAllowance ?
        <Result title="You need to approve the contract to use your OCT token" extra={
          <Button type="primary" key="approve" onClick={incrementsAllowance}>
            Approve
          </Button>
        } /> :
        <Form onFinish={onFinish} labelCol={{ span: 8 }} wrapperCol={{ span: 16 }} 
          initialValues={{ stakeBalance: 10, appchainId }}>
          <Form.Item name="appchainId" label="Appchain Id">
            <Input disabled />
          </Form.Item>
          <Form.Item name="appchainAccount" label="Appchain Account">
            <Input placeholder="please input your account on the appchain"/>
          </Form.Item>
          <Form.Item name="stakeBalance" label="Stake Balance">
            <Input placeholder="The amount you want to stake for" type="number" addonAfter={<TokenBadge />} />
          </Form.Item>
          <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
            <Button type="primary" htmlType="submit" loading={isSubmiting}>Stake</Button>
          </Form.Item>
        </Form>
      }
    </Modal>
  );
}

export default React.memo(StakeModal);