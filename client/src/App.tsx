import React, { useEffect, useState } from "react";
import { AuditLog } from "./AuditLog";
import {
  CreateProductRequest,
  DeleteProductRequest,
  ListProductsRequest,
  Product,
} from "./pb/products_pb";
import { ProductServiceClient } from "./pb/ProductsServiceClientPb";
import { ProductList } from "./ProductList";

const client = new ProductServiceClient("http://localhost:9999");

function App() {
  const [products, setProducts] = useState<Product[]>([]);

  useEffect(() => {
    (async () => {
      let res = await client.listProducts(new ListProductsRequest(), {});
      setProducts(res.getProductsList());
    })();
  }, []);

  const createProduct = async (name: string) => {
    let req = new CreateProductRequest();
    req.setName(name);
    await client.createProduct(req, {});
  };

  const deleteProduct = async (id: string) => {
    let req = new DeleteProductRequest();
    req.setId(id);
    await client.deleteProduct(req, {});
  };

  const addProduct = (product: Product) => {
    setProducts((prev) => [...prev, product]);
  };

  const removeProduct = (id: string) => {
    setProducts((prev) => prev.filter((p) => p.getId() !== id));
  };

  return (
    <div className="container">
      <div className="row">
        <div className="column column-60">
          <h2>Inventory</h2>
          <ProductList
            products={products}
            onCreateProduct={createProduct}
            onDeleteProduct={deleteProduct}
          />
        </div>

        <div className="column column-30 column-offset-10">
          <h2>Activity</h2>
          <AuditLog
            onProductCreated={addProduct}
            onProductDeleted={removeProduct}
          />
        </div>
      </div>
    </div>
  );
}

export default App;
