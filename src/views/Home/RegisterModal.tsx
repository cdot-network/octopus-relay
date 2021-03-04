import React, { useCallback, useState } from "react";

import { Button, Modal, Form, Input } from "antd";

function RegisterModal({ visible, onCancel, onOk }): React.ReactElement {
  const [isSubmiting, setIsSubmiting] = useState<boolean>(false);

  const onFinish = useCallback((values) => {
    setIsSubmiting(true);
    onOk(values);
  }, []);

  return (
    <Modal visible={visible} title="Register Appchain" 
      onCancel={onCancel} destroyOnClose={true} footer={null}>
      <Form onFinish={onFinish} labelCol={{ span: 8 }} wrapperCol={{ span: 16 }} 
        initialValues={{ stakeBalance: 10 }}>
        <Form.Item name="appchainName" label="Appchain Name">
          <Input placeholder="please input the appchain name."/>
        </Form.Item>
        <Form.Item name="runtimeURL" label="Runtime URL">
          <Input placeholder="please input the runtime URL" />
        </Form.Item>
        <Form.Item name="runtimeHash" label="Runtime Hash">
          <Input placeholder="please input the runtime hash" />
        </Form.Item>
        <Form.Item name="stakeBalance" label="Stake Balance">
          <Input placeholder="The amount you want to stake for your chain" type="number" addonAfter="Ⓝ" />
        </Form.Item>
        <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
          <Button type="primary" htmlType="submit" loading={isSubmiting}>Register</Button>
        </Form.Item>
      </Form>
    </Modal>
  );
}

export default React.memo(RegisterModal);