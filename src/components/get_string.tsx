import { useState } from "react";
import { Input, Button, Col, Row } from "antd";
import { invoke } from "@tauri-apps/api/core";


export const GetString = () => {
    const [str, setStr] = useState<string>("");

    // invoke to rust
    const onClick = async () => {
        const res = await invoke("command_get_string", { str });
        console.log(res);
    };

    return (
        <>
            <Row gutter={[16, 16]} style={{ textAlign: "left" }}>
                <Col span={8}>
                    <p>label: </p>
                </Col>
                <Col span={14}>
                    <Input
                        value={str}
                        onChange={(e) => setStr(e.target.value)}
                        placeholder="label"
                    />
                </Col>
                <Col span={8}>
                </Col>
                <Col span={14}>
                    <Button onClick={onClick}>
                        Submit
                    </Button>
                </Col>
            </Row>
        </>
    );
};