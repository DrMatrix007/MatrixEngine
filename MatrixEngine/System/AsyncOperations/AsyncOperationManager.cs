using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.System.AsyncOperations {
    public class AsyncOperationManager {

        private App app;

        private List<AsyncOperation> asyncOperations;

        public AsyncOperationManager(App app) {
            this.app = app;
            asyncOperations = new List<AsyncOperation>();
        }
        public void AddAsyncOperation(AsyncOperation asyncOperation) {
            
            asyncOperations.Add(asyncOperation);

        }

        public void Update() {

            var list = asyncOperations.ToList();

            foreach (var item in list) {
                if (!item.MoveNext()) {
                    asyncOperations.Remove(item);
                }
            }
        }
    }
}
