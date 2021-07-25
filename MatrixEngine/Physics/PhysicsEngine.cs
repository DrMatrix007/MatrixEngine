using MatrixEngine.GameObjects.Components.PhysicsComponents;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using Debug = MatrixEngine.System.Utils;
namespace MatrixEngine.Physics {
    public class PhysicsEngine {

        private List<RigidBodyComponent> dynamicRigidBodies;
        private List<Rect> rectBodies;


        public System.App app
        {
            get;
            private set;
        }

        public PhysicsEngine(System.App app) {
            this.app = app;
            dynamicRigidBodies = new List<RigidBodyComponent>();
            rectBodies = new List<Rect>();
        }

        public void AddNonStaticToFrameComputing(RigidBodyComponent rigidBodyComponent) {

            dynamicRigidBodies.Add(rigidBodyComponent);


        }
        public void AddStaticToFrameComputing(Rect rect) {

        }

        public void Update() {


            foreach (var item in dynamicRigidBodies) {
                if (!item.isStatic) {
                    //TODO: Fix the goddamn drag shit!
                    var multiplier = 1 - item.velocityDrag;
                    if (1 - multiplier <= 0)
                        multiplier = 0;



                    item.velocity += item.gravity;



                    item.position += item.velocity * app.deltaTime;

                    item.velocity = item.velocity * multiplier;


                    //new Vector2f(item.velocity.X - item.velocityDrag.X * app.deltaTime, item.velocity.Y - item.velocityDrag.Y * app.deltaTime);



                }
            }

            //work


            

            var static_list = rectBodies.ToArray();
            var non_static_list = dynamicRigidBodies.ToArray();


            foreach (var @static in static_list) {
                foreach (var nonstatic in non_static_list) {
                    var result = nonstatic.rect.GetCollidingFixFromB(@static);
                    if (result.axis == Physics.CollidingAxis.None) {
                        continue;
                    }
                    if (result.axis == Physics.CollidingAxis.X) {
                        var pos = nonstatic.position;
                        pos.X -= result.fixValue;

                        nonstatic.position = pos;

                        nonstatic.velocity.X = 0;

                    }
                    else if (result.axis == Physics.CollidingAxis.Y) {
                        var pos = nonstatic.position;
                        pos.Y -= result.fixValue;

                        nonstatic.position = pos;

                        nonstatic.velocity.Y = 0;


                    }
                }
            }


            dynamicRigidBodies.Clear();
            rectBodies.Clear();

        }
    }
}
