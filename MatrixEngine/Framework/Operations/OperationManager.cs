using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Framework.Operations {

    public class OperationManager {
        private readonly List<Operation> operations;

        public OperationManager() {
            operations = new List<Operation>();
        }

        public void AddOperation(Operation asyncOperation) {
            operations.Add(asyncOperation);
        }

        public void Update() {
            var list = operations.ToList();

            foreach (var item in list) {
                if (!item.MoveNext()) {
                    operations.Remove(item);
                }
            }
        }
    }
}