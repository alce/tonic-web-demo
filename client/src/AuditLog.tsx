import { ClientReadableStream } from "grpc-web";
import React, { FC, useCallback, useEffect, useState } from "react";
import { Action, Event, SubscribeRequest } from "./pb/audit_log_pb";
import { AuditLogServiceClient } from "./pb/Audit_logServiceClientPb";
import { Product } from "./pb/products_pb";

const client = new AuditLogServiceClient("http://localhost:9999");

interface Props {
  onProductCreated: (product: Product) => void;
  onProductDeleted: (id: string) => void;
}

export const AuditLog: FC<Props> = ({ onProductCreated, onProductDeleted }) => {
  const [messages, setMessages] = useState<string[]>([]);
  const [stream, setStream] = useState<ClientReadableStream<Event> | null>(
    null
  );

  const created = useCallback(onProductCreated, []);
  const deleted = useCallback(onProductDeleted, []);

  useEffect(() => {
    setStream(client.subscribe(new SubscribeRequest()));
  }, []);

  useEffect(() => {
    stream?.on("data", (event: Event) => {
      switch (event.getAction()) {
        case Action.CREATED:
          const product = event.getProduct()!;
          setMessages((prev) => [`${product.getName()} created`, ...prev]);
          created(product);
          break;
        case Action.DELETED:
          const id = event.getProductId();
          setMessages((prev) => [`${id.substr(0, 4)} deleted`, ...prev]);
          deleted(id);
          break;
      }
    });
  }, [stream, created, deleted]);

  return (
    <table>
      <tbody>
        {messages.map((msg, idx) => (
          <tr key={idx}>
            <td>{msg}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
};
