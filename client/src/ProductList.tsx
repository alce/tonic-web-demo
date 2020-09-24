import React, { FC, FormEvent, useState } from "react";
import { Product } from "./pb/products_pb";

interface Props {
  products: Product[];
  onCreateProduct: (name: string) => void;
  onDeleteProduct: (id: string) => void;
}

export const ProductList: FC<Props> = ({
  products,
  onCreateProduct,
  onDeleteProduct,
}) => {
  const [name, setName] = useState("");

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    onCreateProduct(name || "Trinkets");
    setName("");
  };

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <button type="submit">Add product</button>
      </form>

      <table>
        <thead>
          <tr>
            <th>id</th>
            <th>name</th>
            <th> </th>
          </tr>
        </thead>
        <tbody>
          {products.map((p) => (
            <tr key={p.getId()}>
              <td>{p.getId().substr(0, 4)}</td>
              <td>{p.getName()}</td>
              <td>
                <button onClick={() => onDeleteProduct(p.getId())}>x</button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};
