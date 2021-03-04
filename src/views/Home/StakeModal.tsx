import React, { useCallback, useState } from "react";

import { Button, Modal, Form, Input } from "antd";

function StakeModal({ visible, appchainId, onCancel, onOk }): React.ReactElement {
  const [isSubmiting, setIsSubmiting] = useState<boolean>(false);

  const onFinish = useCallback((values) => {
    setIsSubmiting(true);
    onOk(values);
  }, []);

  return (
    <Modal visible={visible} title="Stake" 
      onCancel={onCancel} destroyOnClose={true} footer={null}>
      <Form onFinish={onFinish} labelCol={{ span: 8 }} wrapperCol={{ span: 16 }} 
        initialValues={{ stakeBalance: 10, appchainId }}>
        <Form.Item name="appchainId" label="Appchain Id">
          <Input disabled />
        </Form.Item>
        <Form.Item name="appchainAccount" label="Appchain Account">
          <Input placeholder="please input your account on the appchain"/>
        </Form.Item>
        <Form.Item name="stakeBalance" label="Stake Balance">
          <Input placeholder="The amount you want to stake for" type="number" addonAfter="Ⓝ" />
        </Form.Item>
        <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
          <Button type="primary" htmlType="submit" loading={isSubmiting}>Stake</Button>
        </Form.Item>
      </Form>
    </Modal>
  );
}

export default React.memo(StakeModal);