import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { Button, Col, message, Row, Space, Spin } from "antd";

import { invoke } from "@tauri-apps/api/core";

import { useEffect, useState } from "react";

export const GetPaths = () => {
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [filePaths, setFilePaths] = useState<string[]>([]);
    const [result, setResult] = useState<string>('');


    const invokeTauri = async () => {
        setIsLoading(true);
        // tauri commandからのレスポンスを明瞭なエラーハンドリングで処理
        try {
            const response = await invoke('command_run', { paths: filePaths });
            console.log('tauri command response:', response);
            setResult(response as string);
            setFilePaths([]);
            message.open({
                type: 'success',
                content: 'コマンドの実行に成功しました',
                duration: 3
            })
        } catch (error) {
            console.log('tauri command error:', error);
            setResult(error as string);
            message.open({
                type: 'error',
                content: 'コマンドの実行に失敗しました: ' + error,
                duration: 3
            })
        } finally {
            setIsLoading(false);
        }
    }

    useEffect(() => {
        // ファイルドロップイベントをリッスン
        let unlisten: UnlistenFn;
        const setupListener = async () => {
            // tauri v2指定定数
            unlisten = await listen<string[]>('tauri://drag-drop', (event: any) => {
                console.log('Dropped files:', event.payload.paths);
                const filepath = event.payload.paths;
                if (filepath && filepath.length > 0) {
                    setFilePaths((prev) => [...new Set([...prev, ...filepath])]);
                }
                console.log('Dropped files:', filePaths);

            });
        };

        setupListener();

        return () => {
            if (unlisten) {
                unlisten();
            }
        };
    }, []);

    // 指定インデックスのファイルパスの配列を上に移動
    const up = (index: number) => {
        setFilePaths((prev) => {
            console.log('prev:', prev, 'index:', index);

            if (index <= 0) return prev;

            const newPaths = [...prev];
            const tmp = newPaths[index];
            newPaths[index] = newPaths[index - 1];
            newPaths[index - 1] = tmp;
            return newPaths;
        });
    };

    // ファイルパスを下に移動
    const down = (index: number) => {
        setFilePaths((prev) => {
            console.log('prev:', prev, 'index:', index);

            if (index >= prev.length - 1) return prev;

            const newPaths = [...prev];
            const tmp = newPaths[index];
            newPaths[index] = newPaths[index + 1];
            newPaths[index + 1] = tmp;
            return newPaths;
        });
    };

    // ファイルパスを削除
    const del = (index: number) => {
        console.log('index:', index);
        setFilePaths((prev) => prev.filter((_, i) => i !== index));
    };

    return (
        <div>
            {
                filePaths.length != 0 ? (
                    <>
                        <div style={{ textAlign: 'left', padding: '2rem' }}>
                            <p>PDFパス:</p>
                            <ul>
                                {filePaths.map((path, index) => (
                                    <li key={index}>
                                        <Row gutter={[16, 16]} style={{ width: '100vw' }}>
                                            <Col span={12}>{path}</Col>
                                            <Col span={6}>
                                                <Space>
                                                    <Button type="text" onClick={() => up(index)}>
                                                        ↑
                                                    </Button>
                                                    <Button type="text" onClick={() => down(index)}>
                                                        ↓
                                                    </Button>
                                                    <Button type="text" onClick={() => del(index)}>
                                                        🗑️
                                                    </Button>
                                                </Space>
                                            </Col>
                                        </Row>

                                    </li>
                                ))}
                            </ul>
                        </div>
                        <Button type="primary" onClick={invokeTauri}>
                            実行
                        </Button>
                    </>
                ) : (
                    <p>PDFファイルから、CSVデータを生成します。ウィンドウにPDFファイルをドロップしてください</p>
                )
            }

            <Spin spinning={isLoading}>
                {result ? (
                    <div>
                        <p>結果:</p>
                        <p>{result}</p>
                    </div>
                ) : null}
            </Spin>

        </div>
    );
};
