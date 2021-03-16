import React, { useCallback, useState, useEffect } from "react";

import { Button, Modal, Form, Input, Result, Spin, message, Popconfirm } from "antd";
import Big from 'big.js';

const BOATLOAD_OF_GAS = Big(3).times(10 ** 14).toFixed();

import TokenBadge from "../../components/TokenBadge";

function StakingModal({ visible, appchainId, onCancel, onOk }): React.ReactElement {
 
  const [checkingAllowance, setCheckingAllowance] = useState<boolean>(false);
  const [needIncrementsAllowance, setNeedIncrementsAllowance] = useState(false);

  const [appchain, setAppchain] = useState<any>();
  const [isSubmiting, setIsSubmiting] = useState<boolean>();
  const [unstakingLoading, setUnstakingLoading] = useState<boolean>(false);

  useEffect(() => {
    if (visible) {
      setCheckingAllowance(true);
      Promise.all([
        window.contract.get_appchain({ appchain_id: appchainId }),
        window.tokenContract.get_allowance({
          owner_id: window.accountId,
          escrow_account_id: window.contractName
        })
      ]).then(([appchain, allowance]) => {
        setCheckingAllowance(false);
        setAppchain(appchain);
        if (allowance == 0) {
          setNeedIncrementsAllowance(true);
        }
      }).catch(err => {
        message.error(err.toString());
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

  const onStaking = function(values) {
    const { appchainId, validatorId, offchainWorkerId, stakingAmount } = values;
    setIsSubmiting(true);
    window.contract.staking(
      {
        appchain_id: appchainId,
        id: validatorId,
        ocw_id: offchainWorkerId,
        amount: stakingAmount * 1,
      },
      BOATLOAD_OF_GAS,
      Big(3).times(10 ** 22).toFixed(),
    ).then(() => {
      setIsSubmiting(false);
      window.location.reload();
    }).catch((err) => {
      setIsSubmiting(false);
      message.error(err.toString());
    });
  }

  const onStakingMore = function(values) {
    const { appchainId, stakingAmount } = values;
    setIsSubmiting(true);
    window.contract.staking_more(
      {
        appchain_id: appchainId,
        amount: stakingAmount * 1,
      },
      BOATLOAD_OF_GAS,
      Big(3).times(10 ** 22).toFixed(),
    ).then(() => {
      setIsSubmiting(false);
      window.location.reload();
    }).catch((err) => {
      setIsSubmiting(false);
      message.error(err.toString());
    });
  }

  const unstaking = function(id) {
    setUnstakingLoading(true);
    window.contract.unstaking(
      {
        appchain_id: id,
      },
      BOATLOAD_OF_GAS,
      0
    ).then(() => {
      setUnstakingLoading(false);
      window.location.reload();
    }).catch((err) => {
      setUnstakingLoading(false);
      message.error(err.toString());
    });
  }

  return (
    <Modal visible={visible} title="Staking" 
      onCancel={onCancel} destroyOnClose={true} footer={null}>
      <Spin spinning={checkingAllowance}>
      {
        needIncrementsAllowance ?
        <Result title="You need to approve the contract to use your OCT token" extra={
          <Button type="primary" key="approve" onClick={incrementsAllowance}>
            Approve
          </Button>
        } /> :
        (
          appchain?.validators.some(v => v.account_id == window.accountId) ?
          <div>
            <Form onFinish={onStakingMore} labelCol={{ span: 8 }} wrapperCol={{ span: 16 }} 
              initialValues={{ stakingAmount: 100, appchainId }}>
                <Form.Item name="appchainId" label="Appchain Id">
                  <Input disabled />
                </Form.Item>
                <Form.Item name="stakingAmount" label="Staking Amount">
                  <Input placeholder="The amount you want to staking for" type="number" addonAfter={<TokenBadge />} />
                </Form.Item>
                <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
                  <Button type="primary" htmlType="submit" loading={isSubmiting}>Staking More</Button>
                  <span style={{ margin: "0 10px", color: "#9c9c9c" }}> Or </span>
                  <Popconfirm onConfirm={() => unstaking(appchain.id)} title="Are you sure to unstaking?">
                    <Button type="ghost" loading={unstakingLoading}>Unstaking</Button>
                  </Popconfirm>
                </Form.Item>
            </Form>
          </div> :
          <Form onFinish={onStaking} labelCol={{ span: 8 }} wrapperCol={{ span: 16 }} 
            initialValues={{ stakingAmount: 100, appchainId }}>
            <Form.Item name="appchainId" label="Appchain Id">
              <Input disabled />
            </Form.Item>
            <Form.Item name="validatorId" label="Validator Id">
              <Input placeholder="please input your validator id"/>
            </Form.Item>
            <Form.Item name="offchainWorkerId" label="Offchain Worker Id">
              <Input placeholder="please input your offchain worker id"/>
            </Form.Item>
            <Form.Item name="stakingAmount" label="Staking Amount">
              <Input placeholder="The amount you want to staking for" type="number" addonAfter={<TokenBadge />} />
            </Form.Item>
            <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
              <Button type="primary" htmlType="submit" loading={isSubmiting}>Staking</Button>
            </Form.Item>
          </Form>
        )
      }
      </Spin>
    </Modal>
  );
}

export default React.memo(StakingModal);