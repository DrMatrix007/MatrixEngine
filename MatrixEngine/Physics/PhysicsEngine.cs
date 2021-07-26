using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.MathM;
using MatrixEngine.System;
using SFML.System;
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
            rectBodies.Add(rect);
        }

        public void Update() {


            foreach (var item in dynamicRigidBodies) {
                if (!item.isStatic) {
                    
                    //var multiplier = 1 - item.velocityDrag * app.deltaTime;
                    //multiplier *= app.deltaTime;
                    //if (1 - multiplier <= 0)
                        //multiplier = 0;



                    item.velocity += item.gravity;



                    item.position += item.velocity * app.deltaTime;
                    //item.velocity -= new Vector2f(item.velocity.X>0?item.velocityDrag:-item.velocityDrag, item.velocity.Y > 0 ? item.velocityDrag : -item.velocityDrag)*app.deltaTime;
                    //item.velocity += -1 * item.velocity.Normalize() * item.velocity.Length()*item.velocityDrag;
                    //Utils.Log(item.velocity);
                    var fric = (item.velocity.Length()<1? item.velocity: new Vector2f(item.velocity.X, item.velocity.Y).Normalize()) *(-1)*item.velocityDrag;

                    
                    item.velocity += fric;
                    


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
