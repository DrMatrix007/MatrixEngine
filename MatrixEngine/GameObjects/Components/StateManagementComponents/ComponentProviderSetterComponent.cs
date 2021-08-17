using MatrixEngine.StateManagment;

namespace MatrixEngine.GameObjects.Components.StateManagementComponents {
    public class ComponentProviderSetterComponent<Comp> : Component where Comp : Component {
        private readonly ComponentProvider<Comp> provider;

        public ComponentProviderSetterComponent(ComponentProvider<Comp> provider) {
            this.provider = provider;
        }
        public override void Setup() {
            provider.SetState(GetComponent<Comp>());

        }

        public override void Start() {
        }

        public override void Update() {
            provider.SetState(GetComponent<Comp>());
        }
    }
}
