import React, { useCallback, useEffect, useState } from "react";

import { Button, Modal, Form, Input, Spin, Result } from "antd";

import TokenBadge from "../../components/TokenBadge";
import Big from 'big.js';

function RegisterModal({ visible, onCancel, onOk }): React.ReactElement {
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
    <Modal visible={visible} title="Register Appchain" 
      onCancel={onCancel} destroyOnClose={true} footer={null}>
      <Spin spinning={checkingAllowance}>
        {
          needIncrementsAllowance ?
          <Result title="You need to approve the contract to use your OCT token" extra={
            <Button type="primary" key="approve" onClick={incrementsAllowance}>
              Approve
            </Button>
          } /> :
          <Form onFinish={onFinish} labelCol={{ span: 8 }} wrapperCol={{ span: 16 }} 
            initialValues={{ bondTokenAmount: 100 }}>
            <Form.Item name="appchainName" label="Appchain Name">
              <Input placeholder="please input the appchain name."/>
            </Form.Item>
            <Form.Item name="runtimeURL" label="Runtime URL">
              <Input placeholder="please input the runtime URL" />
            </Form.Item>
            <Form.Item name="runtimeHash" label="Runtime Hash">
              <Input placeholder="please input the runtime hash" />
            </Form.Item>
            <Form.Item name="bondTokenAmount" label="Bond Token">
              <Input placeholder="The amount you want to stake for your chain" type="number" addonAfter={<TokenBadge />} />
            </Form.Item>
            <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
              <Button type="primary" htmlType="submit" loading={isSubmiting}>Register</Button>
            </Form.Item>
          </Form>
        }
      </Spin>
    </Modal>
  );
}

export default React.memo(RegisterModal);