using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.StateManagment;

namespace MatrixEngine.GameObjects.Components.StateManagementComponents {
    public class ConsumerComponent<Output> : Component {
        public ConsumerComponent(Provider<Output> provider)  {
            this.provider = provider;
        }

        public Provider<Output> provider { get; }

        public override void Start() {
        }

        public override void Update() {
        }
        public Output GetOutput() {
            return provider.data;
        }
    }
}
